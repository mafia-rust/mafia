


export default class AudioController {
    static audio = new Audio();
    static queue: string[] = []
    static playing: boolean = false;

    static setVolume(volume: number) {
        this.audio.volume = volume;
    }

    static playQueue() {
        if(this.queue.length > 0) {

            this.playing = true;
            this.playFile(this.queue[0], () => {
                this.queue = this.queue.slice(1)
                this.playQueue();
            });
        }else{
            this.playing = false;
        }
    }
    static clearQueue() {
        this.queue = [];
        this.pause();
    }
    static playFile(src: string, onEnd?: () => void){
        this.audio.pause();
        this.audio.src = src;
        this.audio.load();

        const onEnded = () => {
            if(onEnd !== undefined) onEnd();
            this.audio.removeEventListener("ended", onEnded);
        }

        this.play();
        this.audio.addEventListener("ended", onEnded);
    }
    static queueFile(src: string) {
        this.queue.push(src);
    }
    static play() {
        let playPromise = this.audio.play();
        playPromise.then(() => {

            this.audio.currentTime = 0;
            this.audio.playbackRate = 1;
        }).catch((error) => {
            console.log("Audio failed to play: " + error);
        }); 
    }
    static pause() {
        this.audio.pause();
    }
}