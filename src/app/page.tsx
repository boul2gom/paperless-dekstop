"use client"

import { Navigation } from '@/src/components/Layout/Navigation/Navigation';
import { HeaderBar } from '@/src/components/Layout/HeaderBar/HeaderBar';

import classes from '@/src/styles/Main.module.css';
import { ScrollArea } from "@mantine/core";
import { CarouselSkeleton } from '../components/Carousel';
import { Suspense } from 'react';
import { SWRConfig } from 'swr';
import dynamic from 'next/dynamic';

const Logo = dynamic(() => import('@/src/components/Layout/Navigation/Logo'), {
  suspense: true,
  ssr: false,
})

const Carousel = dynamic(() => import('@/src/components/Carousel'), {
  suspense: true,
  ssr: false,
})

export default function Page() {
  return (
    <div className={classes.page_container}>
      <ScrollArea type="never">
      <SWRConfig value={{ suspense: true }}>
        <div className={classes.navigation_container}>
          <Logo />
          <Navigation />
        </div>

        <div className={classes.top_container}>
          <HeaderBar />
        </div>

        <div className={classes.content_container}>
          <Suspense fallback={<CarouselSkeleton />}>
            <Carousel />
          </Suspense>
        </div>  
      </SWRConfig>
      </ScrollArea>
    </div>
  );
}