[Website](https://intx4.github.io) 

# TODO
```
 <main class={format!("{} min-h-screen", theme.current)}> // Ensure main has min height of the screen
                <div class="w-full min-h-screen bg-gray-50 dark:bg-slate-900 text-black dark:text-slate-300 transition flex flex-col justify-between">
                    <div class="max-w-[1200px] m-auto p-4 flex-grow"> // Add flex-grow to allow it to stretch
                        <Nav />
                        <Landing />
                        <Aboutme />
                        <Contact />
                    </div>
                </div>
            </main>
```
fix this on mobile (the black background is only half on mobile)