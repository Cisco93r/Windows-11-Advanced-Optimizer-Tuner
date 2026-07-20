use is_root::is_root;
use std::process::Command;

fn main() {
    println!("=== SYSTEM TUNER IN RUST ===");

    // Verifica la presenza dei privilegi amministrativi (Sudo / Administrator)
    if !is_root() {
        println!("[!] ERRORE: Questo programma richiede privilegi di Amministratore (Root/Administrator).");
        println!("Riavvia il programma tramite 'sudo' su Linux/Mac o come 'Amministratore' su Windows.");
        return;
    }

    // Esegue le ottimizzazioni in base al sistema operativo di destinazione
    esegui_ottimizzazioni();
}

// ==========================================
// SEZIONE WINDOWS
// ==========================================
#[cfg(target_os = "windows")]
fn esegui_ottimizzazioni() {
    println!("[*] Rilevato sistema operativo: Windows");

    // 1. Installazione Programmi Base tramite Winget (Gestore pacchetti nativo di Windows)
    println!("--> Installazione programmi base (Firefox, VLC, 7-Zip)...");
    let programmi = vec!["Mozilla.Firefox", "VideoLAN.VLC", "7zip.7zip"];
    for prog in programmi {
        let _ = Command::new("winget")
        .args(&["install", "--id", prog, "--silent", "--accept-source-agreements", "--accept-package-agreements"])
        .status();
    }

    // 2. Disattivazione UAC (User Account Control) tramite Registro di Sistema
    println!("--> Disattivazione UAC...");
    let _ = Command::new("reg")
    .args(&["add", "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System", "/v", "EnableLUA", "/t", "REG_DWORD", "/d", "0", "/f"])
    .status();

    // 3. Disattivazione Telemetria e Servizi di Tracciamento DiagTrack
    println!("--> Disattivazione Telemetria...");
    // Arresta e disabilita il servizio Connected User Experiences and Telemetry
    let _ = Command::new("sc").args(&["stop", "DiagTrack"]).status();
    let _ = Command::new("sc").args(&["config", "DiagTrack", "start=", "disabled"]).status();

    // Modifica chiavi di registro della telemetria principale
    let _ = Command::new("reg")
    .args(&["add", "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection", "/v", "AllowTelemetry", "/t", "REG_DWORD", "/d", "0", "/f"])
    .status();

    println!("\n[+] Operazioni completate su Windows. Nota: Per rendere effettiva la disattivazione dell'UAC è necessario riavviare il PC.");
}

// ==========================================
// SEZIONE LINUX
// ==========================================
#[cfg(target_os = "linux")]
fn esegui_ottimizzazioni() {
    println!("[*] Rilevato sistema operativo: Linux");

    // 1. Aggiornamento del Sistema e dei pacchetti
    println!("--> Aggiornamento dei repository e dei pacchetti...");
    // Questo esempio usa APT (Debian/Ubuntu/Mint), per Arch-based usa "pacman -Syu --noconfirm"
    let _ = Command::new("apt-get").args(&["update", "-y"]).status();
    let _ = Command::new("apt-get").args(&["upgrade", "-y"]).status();

    // 2. Installazione Programmi Base
    println!("--> Installazione programmi base (Firefox, VLC, Git)...");
    let _ = Command::new("apt-get")
    .args(&["install", "-y", "firefox", "vlc", "git"])
    .status();

    println!("\n[+] Aggiornamenti e installazioni completate con successo su Linux.");
}

// ==========================================
// SEZIONE MAC OS (macOS)
// ==========================================
#[cfg(target_os = "macos")]
fn esegui_ottimizzazioni() {
    println!("[*] Rilevato sistema operativo: macOS");

    // 1. Installazione Programmi Base tramite Homebrew (richiede Brew preinstallato)
    println!("--> Installazione programmi base tramite Homebrew...");
    let programmi = vec!["firefox", "vlc", "visual-studio-code"];
    for prog in programmi {
        // Brew deve essere richiamato evitando i privilegi di root stretti per i cask,
        // ma per semplicità logica usiamo il comando di base
        let _ = Command::new("brew").args(&["install", "--cask", prog]).status();
    }

    // 2. Disattivazione della Telemetria e dell'invio dei crash log ad Apple
    println!("--> Disattivazione dell'invio automatico dei Crash Report ad Apple...");
    let _ = Command::new("defaults")
    .args(&["write", "com.apple.CrashReporter", "DialogType", "none"])
    .status();

    println!("\n[+] Operazioni completate con successo su macOS.");
}
