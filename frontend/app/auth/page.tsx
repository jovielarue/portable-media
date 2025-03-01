"use client";
import { useState } from "react";

export default function Page() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [login, setLogin] = useState<boolean>(true);
  const [alert, setAlert] = useState<string>("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    setAlert("");
    try {
      const userData = new FormData();
      userData.set("username", username);
      userData.set("password", password);
      const res = await fetch(
        `http://localhost:8000/auth/${login ? "login" : "signup"}`,
        {
          body: userData,
          method: "POST",
        },
      );

      console.log(res);

      if (!res.ok) {
        setAlert(
          "There was an error authenticating your user. Here's the issue: " +
            res.statusText,
        );
      }

      setUsername("");
      setPassword("");
    } catch (e) {
      console.error("There was an error authing this user: " + e);
    }
  };

  const handleSetLogin = () => {
    if (login) {
      setLogin(false);
    } else {
      setLogin(true);
    }
  };

  return (
    <div className={"flex flex-col items-center gap-5"}>
      <h1 className={"text-2xl font-bold -mt-[25%]"}>
        please {login ? "log in" : "sign up"}
      </h1>
      <form
        className={"flex flex-col gap-3 w-56"}
        onSubmit={(e) => handleSubmit(e)}
      >
        <div className={"flex flex-col gap-1"}>
          <label htmlFor="username">username</label>
          <input
            className={
              "bg-secondary text-background placeholder-background p-2 rounded-sm"
            }
            name="username"
            type="text"
            placeholder="username goes here..."
            value={username}
            onChange={(e) => {
              setUsername(e.target.value);
            }}
          />
        </div>
        <div className={"flex flex-col gap-1"}>
          <label htmlFor="password">password</label>
          <input
            className={
              "bg-secondary text-background placeholder-background p-2 rounded-sm"
            }
            name="password"
            type="password"
            placeholder="password goes here..."
            value={password}
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
        </div>
        <button
          type="submit"
          className={
            "hover:bg-accent2 bg-secondary hover:text-background text-accent py-1 px-3 rounded-sm"
          }
        >
          {login ? "log in" : "sign up"}
        </button>
        <button
          type="button"
          className={
            "hover:bg-primary bg-secondary hover:text-background text-accent py-1 px-3 rounded-sm text-sm"
          }
          onClick={handleSetLogin}
        >
          {login ? "don't have an account?" : "already have an account?"}
        </button>
      </form>
      {alert !== "" && (
        <div
          className={
            "flex items-center justify-center bg-accent2 text-background p-5 text-center rounded-sm w-80"
          }
        >
          {alert}
        </div>
      )}
    </div>
  );
}
