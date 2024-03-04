import '@mantine/core/styles.css';
import '@mantine/carousel/styles.css';
import '@mantine/spotlight/styles.css';

import { MantineProvider } from '@mantine/core';

import { Navigation } from '@/components/Layout/Navigation/Navigation';

import classes from '@/styles/Main.module.css';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

import { ScrollArea } from "@mantine/core";
import { HeaderBar } from '@/components/Layout/HeaderBar/HeaderBar';
import Carousel from '@/components/Carousel';

const query_client = new QueryClient({
  defaultOptions: { queries: {
      retry: 0,
  }},
});

function App() {
  return (
    <MantineProvider>
    <QueryClientProvider client={query_client}>
    <div className={classes.page_container}>
      <ScrollArea type="never">
        <div className={classes.navigation_container}>
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