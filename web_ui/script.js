const CLIENT_ID = '964113780245-bkk2i4tf7ih5joqvltkg2ma63lodks20.apps.googleusercontent.com';
const API_URL = 'http://localhost:8081';

// Check if backend is running
async function checkBackend() {
    try {
        const response = await fetch(`${API_URL}/users`);
        console.log('Backend is running');
        return true;
    } catch (error) {
        console.error('Backend not reachable:', error);
        alert('Backend server not running. Please start the Rust API on port 8081.');
        return false;
    }
}

function initializeGoogleSignIn() {
    google.accounts.id.initialize({
        client_id: CLIENT_ID,
        callback: handleCredentialResponse
    });
}

async function handleCredentialResponse(response) {
    try {
        const userInfo = parseJwt(response.credential);
        
        const authData = {
            google_id: userInfo.sub,
            name: userInfo.name,
            email: userInfo.email,
            picture_url: userInfo.picture
        };

        const apiResponse = await fetch(`${API_URL}/google-auth`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(authData)
        });

        const result = await apiResponse.json();
        
        showUserInfo(userInfo);
        alert(result.message || 'Login successful!');
        
    } catch (error) {
        console.error('Authentication error:', error);
        alert('Authentication failed: ' + error.message);
    }
}

function parseJwt(token) {
    const base64Url = token.split('.')[1];
    const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
    const jsonPayload = decodeURIComponent(atob(base64).split('').map(function(c) {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
    }).join(''));
    return JSON.parse(jsonPayload);
}

function showUserInfo(userInfo) {
    localStorage.setItem('userData', JSON.stringify(userInfo));
    window.location.href = 'dashboard.html';
}

window.onload = function() {
    checkBackend();
    initializeGoogleSignIn();
};