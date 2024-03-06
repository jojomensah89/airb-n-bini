import { getKindeServerSession } from "@kinde-oss/kinde-auth-nextjs/server";
import { NextResponse } from "next/server";
import { unstable_noStore as noStore } from "next/cache";
import axios from "axios";

export async function GET() {
  noStore();
  const { getUser } = getKindeServerSession();

  const user = await getUser();

  if (!user || user === null || !user.id) {
    throw new Error("Smoething went wrong, i am sorry....");
  }
  const url = "http://localhost:8080/user/create";

  const user_data = {
    id: user.id,
    first_name: user.given_name?.toLowerCase() ?? "",
    last_name: user.family_name?.toLowerCase() ?? "",
    email: user.email ?? "",
    profile_image: user.picture ?? `https://avatar.vercel.sh/${user.given_name}`,
  };

  try {
    const response = await fetch(url, {
      method: "POST", // Specify POST method
      headers: {
        "Content-Type": "application/json", // Send data as JSON
      },
      body: JSON.stringify(user_data), // Convert data to JSON string
    });

    console.log(JSON.stringify(user_data));
    if (!response.ok) {
            console.log(response.statusText);

      return NextResponse.json({ error: response.statusText }, { status: 400 });
    }
  } catch (err) {
    return NextResponse.json(
      { error: "Internal Server Error again" },
      { status: 500 }
    );
  }

  return NextResponse.redirect("http://localhost:3000");
}
