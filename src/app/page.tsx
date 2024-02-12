"use client"

import { Navigation } from '@/src/components/Layout/Navigation/Navigation';
import { HeaderBar } from '@/src/components/Layout/HeaderBar/HeaderBar';

import classes from '@/src/styles/Main.module.css';
import { Logo } from "@/src/components/Layout/Navigation/Logo";
import { ScrollArea } from "@mantine/core";
import { Carousel, CarouselSkeleton } from '../components/Carousel';
import { Suspense } from 'react';

export default function Page() {
  return (
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
          <Suspense fallback={<CarouselSkeleton from="Main page" />}>
            <Carousel />
          </Suspense>
        </div>  
      </ScrollArea>
    </div>
  );
}