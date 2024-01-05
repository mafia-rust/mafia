import React from "react";
import "../index.css";
import "./anchor.css";
import GAME_MANAGER from "..";
import translate from "../game/lang";
import Settings from "./Settings";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    mobile: boolean,
    content: JSX.Element,
    error: JSX.Element | null,
    rejoinCard: JSX.Element | null,

    settings: JSX.Element | null,
    volume: number,
    audio: HTMLAudioElement
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        this.state = {
            mobile: false,
            content: this.props.content,
            error: null,
            rejoinCard: null,

            settings: null,
            volume: .5,
            audio: new Audio()
        }
        this.state.audio.addEventListener("ended", () => {
            console.log("Playing audio: " + Anchor.instance.state.audio.src);
            let playPromise = Anchor.instance.state.audio.play();
            playPromise.then(() => {
            }).catch((error) => {
                console.log("Audio failed to play: " + error);
            });
        });
    }
    
    componentDidMount() {
        Anchor.instance = this;

        window.addEventListener("resize", Anchor.onResize);
        Anchor.onResize();

        this.props.onMount()
    }
    componentWillUnmount() {
        window.removeEventListener("resize", Anchor.onResize);
    }
    
    private static onResize() {
        const mobile = window.innerWidth <= 600;
        if (Anchor.instance.state.mobile && !mobile) {
            console.info("Switching to desktop layout");
        } else if (mobile && !Anchor.instance.state.mobile) {
            console.info("Switching to mobile layout");
        }
        Anchor.instance.setState({mobile});
    }
    
    handleRejoin(roomCode: string, playerId: number) {
        this.setState({rejoinCard: null});
        GAME_MANAGER.sendRejoinPacket(roomCode, playerId);
        console.log("Attempting rejoining game: " + roomCode + " " + playerId);
    }
    handleCancelRejoin() {
        this.setState({rejoinCard: null});
        GAME_MANAGER.deleteReconnectData();
    }

    static playAudioFile(src: string | null, timeLeftSeconds: number | undefined = undefined) {
        Anchor.instance.state.audio.pause();
        if(src === null) return;
        Anchor.instance.setState({audio: new Audio(src)}, () => {
            console.log("Playing audio: " + Anchor.instance.state.audio.src);
            Anchor.startAudio(timeLeftSeconds);
        });
    }
    static startAudio(timeLeftSeconds: number | undefined = undefined) {
        let playPromise = Anchor.instance.state.audio.play();
        playPromise.then(() => {

            // Anchor.instance.state.audio.duration;
            // Anchor.instance.state.audio.currentTime = 45;
            // Anchor.instance.state.audio.playbackRate = 2;
            if(Anchor.instance.state.audio.duration !== Infinity && !Number.isNaN(Anchor.instance.state.audio.duration)){
                let startTime = Math.ceil(Anchor.instance.state.audio.duration - (timeLeftSeconds ?? 0));
                if (startTime > 0 && startTime < Anchor.instance.state.audio.duration) {
                    console.log("Starting audio at " + startTime + " seconds")
                    Anchor.instance.state.audio.currentTime = startTime;
                };
            }
        }).catch((error) => {
            console.log("Audio failed to play: " + error);
        });
        
            
    }
    static stopAudio() {
        Anchor.instance.state.audio.pause();
    }

    render(){
        return <div className="anchor">
            {this.state.content}
            {this.state.error}
            {this.state.rejoinCard}
            {this.state.settings}
            {/** Next line is openSettings button*/}
            <button className="material-icons-round settings-button" onClick={() => Anchor.openSettings()}>settings</button>
        </div>
    }

    public static setContent(content: JSX.Element){
        Anchor.instance.setState({content : content});
    }
    public static pushError(title: string, body: string) {
        Anchor.instance.setState({error: <ErrorCard
            onClose={() => Anchor.instance.setState({ error: null })}
            error={{title, body}}
        />});
    }
    public static pushRejoin(roomCode: string, playerId: number) {
        Anchor.instance.setState({rejoinCard:
            <div className="error-card slide-in">
                <header><button onClick={() => {Anchor.instance.handleRejoin(roomCode, playerId)}}>{translate("menu.play.button.rejoin")}</button></header>
                <button onClick={() => {Anchor.instance.handleCancelRejoin()}}>✕</button>
                <div></div>
            </div>
        });
    }
    public static openSettings() {
        Anchor.instance.setState({settings:
            <Settings 
                volume={Anchor.instance.state.volume} 
                onVolumeChange={(volume) => {
                    Anchor.instance.setState({volume: volume});
                    Anchor.instance.state.audio.volume = volume
                }}
            />
        });
    }

    public static isMobile(): boolean {
        return Anchor.instance.state.mobile;
    }
}

type Error = {
    title: string,
    body: string
}

function ErrorCard(props: { error: Error, onClose: () => void }) {
    return <div className="error-card slide-in" onClick={() => props.onClose()}>
        <header>{props.error.title}</header>
        <button>✕</button>
        <div>{props.error.body}</div>
    </div>
}
