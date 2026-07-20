# Windows-11-Advanced-Optimizer-Tuner
Sviluppato e cross-compilato con ❤️ su Linux.

# Windows 11 Advanced Optimizer & Tuner 🚀

Un tool automatizzato scritto in **Rust** per configurare rapidamente macchine Windows 11 appena installate, ottimizzare la privacy e installare il software di base in modalità totalmente silenziosa.

## ⚡ Caratteristiche principali
- **Aggiornamento di Sistema:** Cerca e installa gli aggiornamenti di sistema pendenti tramite `winget`.
- **Software di Base:** Installa in modo invisibile Firefox, VLC, 7-Zip, VS Code e Proton VPN.
- **Privacy Core:** Interrompe e disabilita i servizi di telemetria e tracciamento background (`DiagTrack`, `dmwappushservice`).
- **Rimozione Restrizioni:** Disattiva i prompt invasivi dell'UAC modificando in sicurezza le chiavi di registro.

## 🛠️ Come eseguire l'eseguibile precompilato
1. Scarica il file `system_tuner_win.exe`.
2. Apri **PowerShell** o il **Prompt dei comandi** come **Amministratore**.
3. Naviga fino alla cartella del file e lancialo:
   ```powershell
   .\system_tuner_win.exe
