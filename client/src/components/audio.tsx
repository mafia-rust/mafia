//dont forget, if there is less than 3 people alive then the game goes 2x speed
const phase_to_audio = {
    "morning": null,
    "discussion": "08. Heated.mp3",
    "voting": "16. Suspicion.mp3",
    "testimony": "11. Innocence.mp3",
    "judgement": null,
    "finalWords": null,
    "night": "17. What Lurks In The Night.mp3",

    "lobby": "01. Calm Before The Storm.mp3",
    "nameSelect": "02. Who Am I.mp3",
}

export function getAudioSrcFromString(str: string): string | null{
    let audio = phase_to_audio[str as keyof typeof phase_to_audio];
    if(audio){
        return "/audio/" + audio;
    }
    return null;
}

