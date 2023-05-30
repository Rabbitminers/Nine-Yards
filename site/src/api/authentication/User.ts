import { jsonHeader } from "../helpers/RequestHelpers";

export async function signup(username: String, email: String, password: String): Promise<void> {
    const body = JSON.stringify({
        "username": username,
        "email": email,
        "password": password
    })

    const requestOptions: RequestInit = {
        method: 'POST',
        headers: jsonHeader,
        body: body,
        redirect: 'follow'
    };
      
    fetch("http://127.0.0.1:8000/api/auth/signup", requestOptions)
        .then(response => response.text())
        .then(result => console.log(result))
        .catch(error => {
            console.error('Failed to create account: ', error)
        });
}

export function login(): void {

}