const API_URL = 'http://localhost:8081';

document.addEventListener('DOMContentLoaded', function() {
    loadUserData();
    loadStats();
    
    document.getElementById('logout-btn').addEventListener('click', logout);
});

function loadUserData() {
    const userData = JSON.parse(localStorage.getItem('userData'));
    if (!userData) {
        window.location.href = 'index.html';
        return;
    }
    
    // Header user info
    document.getElementById('user-name').textContent = userData.name;
    document.getElementById('user-avatar').src = userData.picture || 'https://via.placeholder.com/40';
    
    // Profile card info
    document.getElementById('profile-name').textContent = userData.name;
    document.getElementById('profile-email').textContent = userData.email;
    
    // Stats
    const loginTime = new Date().toLocaleString();
    document.getElementById('login-time').textContent = loginTime;
    document.getElementById('activity-time').textContent = loginTime;
}

async function loadStats() {
    try {
        const response = await fetch(`${API_URL}/users`);
        const users = await response.json();
        document.getElementById('total-users').textContent = users.length;
    } catch (error) {
        document.getElementById('total-users').textContent = 'Error loading';
    }
}

function logout() {
    localStorage.removeItem('userData');
    window.location.href = 'index.html';
}