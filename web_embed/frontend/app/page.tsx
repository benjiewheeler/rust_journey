import Image from "next/image";

export default function Home() {
    return (
        <div className="grid min-h-screen justify-center">
            <main className="row-start-2 flex flex-col items-center sm:items-start">
                <Image className="dark:invert" src="/next.svg" alt="Next.js logo" width={180} height={40} priority />
            </main>
        </div>
    );
}
