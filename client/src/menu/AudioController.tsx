export default class AudioController {
    private static audio = new Audio();
    private static queue: string[] = []
    private static autoplay: boolean = true;
    private static currentlyPlaying: boolean = false;

    public static setVolume(volume: number) {
        this.audio.volume = volume;
    }

    public static unpauseQueue() {
        if (!this.currentlyPlaying) {
            this.autoplay = true;
            this.playQueue()
        }
    }
    public static pauseQueue() {
        this.autoplay = false;
        this.audio.pause();
    }
    public static clearQueue() {
        this.queue = [];
    }
    public static queueFile(src: string) {
        this.queue.push(src);
        if (this.autoplay) {
            this.unpauseQueue();
        }
    }

    private static playQueue() {
        if(this.queue.length > 0) {
            this.currentlyPlaying = true;
            console.log(`Playing ${this.queue[0]}`);
            this.playFile(this.queue[0], () => {
                this.queue = this.queue.slice(1)
                this.currentlyPlaying = false;
                if (this.autoplay)
                    this.playQueue();
            });
        }
    }
    private static playFile(src: string, onEnd?: () => void){
        this.audio.pause();
        this.audio.src = src;
        this.audio.load();

        const onEnded = () => {
            if(onEnd !== undefined) onEnd();
            this.audio.removeEventListener("ended", onEnded);
        }

        this.playAudio();
        this.audio.addEventListener("ended", onEnded);
    }
    private static playAudio() {
        this.audio.play()
            .then(() => {
                this.audio.currentTime = 0;
                this.audio.playbackRate = 1;
            }).catch((error) => {
                console.log("Audio failed to play: " + error);
            }); 
    }
}