import '@mantine/core/styles.css';
import '@mantine/carousel/styles.css';
import '@mantine/spotlight/styles.css';

import { MantineProvider } from '@mantine/core';

import { Navigation } from '@/src/components/Layout/Navigation/Navigation';
import { HeaderBar } from '@/src/components/Layout/HeaderBar/HeaderBar';

import classes from '@/src/styles/Main.module.css';
import { ScrollArea } from "@mantine/core";
import Carousel from '../components/Carousel';
import Logo from "@/src/components/Layout/Navigation/Logo";

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const query_client = new QueryClient({
  defaultOptions: { queries: {
      retry: 0,
  }},
});

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <MantineProvider>
    <QueryClientProvider client={query_client}>
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>

    <div className={classes.page_container}>
      <ScrollArea type="never">
        <div className={classes.navigation_container}>
          <Logo />
          <Navigation />
        </div>

        <div className={classes.top_container}>
          <HeaderBar />
        </div>

        <div className={classes.content_container}>
          <Carousel />
        </div>
      </ScrollArea>
    </div>
    <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
    </MantineProvider>
  );
}

export default App;