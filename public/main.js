function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
}

if (!getCookie('auth_token')) {
    const token = prompt('Please enter your auth token:');
    if (token) {
        document.cookie = `auth_token=${token};path=/`;
    } else {
        alert('You need a token to proceed.');
    }
}