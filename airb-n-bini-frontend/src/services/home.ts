import axios from "axios";

const url = `http://localhost:8080`;

interface CreateHome {
    title:string,
    description:string,
    price:number,
    image:string,
    
}
async function httpCreateAirbnbHome(home: CreateHome) {
  try {
    const insertUser = await axios.post(url, home);
    return insertUser;
  } catch (err) {
    console.error(err);
  }
}

export { httpCreateAirbnbHome };
