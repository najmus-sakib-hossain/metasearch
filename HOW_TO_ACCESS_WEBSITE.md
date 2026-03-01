# 🌐 How to Access Your Metasearch Website

## ❌ The Problem

You're trying to access: `http://0.0.0.0:8888/` (if you were)

This won't work! `0.0.0.0` is a special address that servers use to listen on all network interfaces, but browsers can't connect to it.

## ✅ The Solution

Use one of these URLs instead:

### Option 1: localhost (RECOMMENDED)
```
http://localhost:8888/
```

### Option 2: 127.0.0.1
```
http://127.0.0.1:8888/
```

### Option 3: Your Local IP (for other devices)
```
http://192.168.x.x:8888/
```
(Replace with your actual local IP address)

## 🔍 How to Find Your Local IP

### Windows
```bash
ipconfig
```
Look for "IPv4 Address" under your active network adapter.

### Linux/Mac
```bash
ifconfig
# or
ip addr show
```

## ✅ Verification

The server IS running correctly:
- ✅ Listening on port 8888
- ✅ Serving requests
- ✅ 208 engines registered
- ✅ Search functionality working

Just use the correct URL in your browser!

## 🚀 Quick Test

Open your browser and go to:
```
http://localhost:8888/
```

You should see the Metasearch homepage with a search box.

## 🔧 If It Still Doesn't Work

1. **Check if server is running:**
   ```bash
   curl http://localhost:8888/
   ```
   Should return HTML.

2. **Check firewall:**
   - Windows: Allow port 8888 in Windows Firewall
   - Antivirus: May be blocking the connection

3. **Try different browser:**
   - Chrome, Firefox, Edge, etc.

4. **Restart server:**
   ```bash
   # Stop current server (Ctrl+C)
   # Start again
   target/release/metasearch.exe
   ```

## 📝 Summary

**Correct URL:** `http://localhost:8888/` ✅

The server is working perfectly. Just use `localhost` instead of `0.0.0.0` in your browser!
