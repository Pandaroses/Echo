import {FormEvent, useState} from "react";

function Login() {
    const [email , setEmail] = useState("");
    const [password , setPassword] = useState("");

    async function handleSubmit(e : FormEvent<HTMLFormElement>) {
        e.preventDefault();
        const real = JSON.stringify({
            "email" : email,
            "password" : password
        })
        console.log(real)
        const res = await fetch("/auth/login" , {
            method : "POST",
            headers : new Headers({
                "content-type" : "application/json",
            }),
            credentials : "same-origin",
            body : real
        });
        console.log(res.status)
    }
    return <div>
        <h1> Login </h1>
        <form onSubmit={handleSubmit}>
            <input value={email} placeholder="email" onChange={e => setEmail(e.target.value)} type="email" required/>
            <input value={password} placeholder="password" onChange={e => setPassword(e.target.value)} type="password" required/>
            <button type="submit"> Login </button>
        </form>
    </div>
}
export default Login;