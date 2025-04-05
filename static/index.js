
const allCookies = document.cookie;
const cookies = allCookies.split(";");
for (let i = 0; i < cookies.length; i++) {
    console.log(cookies[i].trim());
}

