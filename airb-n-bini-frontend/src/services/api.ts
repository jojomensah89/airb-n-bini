import axios from "axios";

const url = `http://localhost:8080`;

interface CreateUser {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  profile_image:string;
}
async function httpCreateUser(user: CreateUser) {
  try {
    const insertUser = await axios.post(url, user);
    return insertUser.status
  } catch (err) {
    console.error(err);
  }
}



export { httpCreateUser };