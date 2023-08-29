import {useState} from "react";
function Home() {
    const [email , _] = useState("");
    async function handleClick(e : any) {
        e.preventDefault();
        const res = await fetch("/account/details" , {
            credentials : "same-origin",
        })
        console.log(await res.json())
    }
    return <div>
        <h1> Button </h1>
        <button onClick={handleClick}> Get dets </button>
        <h1> {email} </h1>
    </div>
}
export default Home;