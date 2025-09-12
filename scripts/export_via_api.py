#!/usr/bin/env python3
"""
Export current CMS state via API calls
This script connects to the running backend API to export all data
"""

import requests
import json
import os
from datetime import datetime

# Configuration
BACKEND_URL = "http://localhost:8081"
EXPORT_DIR = "./default_data_export"

def login_and_get_token():
    """Login as admin and get JWT token"""
    login_data = {
        "username": "admin",
        "password": "admin"
    }
    
    response = requests.post(f"{BACKEND_URL}/api/auth/login", json=login_data)
    if response.status_code == 200:
        return response.json().get("token")
    else:
        print(f"Login failed: {response.status_code} - {response.text}")
        return None

def export_data(token, endpoint, filename):
    """Export data from an API endpoint"""
    headers = {"Authorization": f"Bearer {token}"}
    
    try:
        response = requests.get(f"{BACKEND_URL}/api/{endpoint}", headers=headers)
        if response.status_code == 200:
            data = response.json()
            
            # Save as JSON
            with open(f"{EXPORT_DIR}/{filename}.json", 'w') as f:
                json.dump(data, f, indent=2)
            
            print(f"✅ Exported {endpoint} -> {filename}.json ({len(data) if isinstance(data, list) else 1} items)")
            return data
        else:
            print(f"❌ Failed to export {endpoint}: {response.status_code} - {response.text}")
            return None
    except Exception as e:
        print(f"❌ Error exporting {endpoint}: {e}")
        return None

def main():
    print("🚀 Exporting current CMS state via API...")
    
    # Create export directory
    os.makedirs(EXPORT_DIR, exist_ok=True)
    
    # Login and get token
    token = login_and_get_token()
    if not token:
        print("❌ Could not authenticate. Make sure backend is running and admin user exists.")
        return
    
    print("✅ Successfully authenticated")
    
    # Export all data
    endpoints = [
        ("posts", "posts"),
        ("pages", "pages"),
        ("categories", "categories"),
        ("navigation", "navigation"),
        ("settings", "settings"),
        ("component-templates", "component_templates"),
        ("plugins", "plugins")
    ]
    
    exported_data = {}
    
    for endpoint, filename in endpoints:
        data = export_data(token, endpoint, filename)
        if data is not None:
            exported_data[filename] = data
    
    # Create a combined export file
    with open(f"{EXPORT_DIR}/complete_export.json", 'w') as f:
        json.dump(exported_data, f, indent=2)
    
    print(f"\n📁 Export completed! Files saved to {EXPORT_DIR}/")
    print("📋 Exported data:")
    for key, data in exported_data.items():
        count = len(data) if isinstance(data, list) else 1
        print(f"  - {key}: {count} items")

if __name__ == "__main__":
    main()
