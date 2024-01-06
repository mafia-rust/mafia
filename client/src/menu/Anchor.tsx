import React from "react";
import "../index.css";
import "./anchor.css";
import GAME_MANAGER from "..";
import translate from "../game/lang";
import Settings, { DEFAULT_SETTINGS } from "./Settings";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    mobile: boolean,
    content: JSX.Element,
    error: JSX.Element | null,
    rejoinCard: JSX.Element | null,

    settings_menu: boolean,
    volume: number,
    audio: HTMLAudioElement
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        let settings_obj = GAME_MANAGER.loadSettings();
        //set default settings
        if(settings_obj === null){
            settings_obj = DEFAULT_SETTINGS;
        }

        this.state = {
            mobile: false,
            content: this.props.content,
            error: null,
            rejoinCard: null,

            settings_menu: false,
            volume: settings_obj.volume,
            audio: new Audio()
        }
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

    static playAudioFile(src: string | null) {
        Anchor.instance.state.audio.pause();
        if(src === null) return;
        Anchor.instance.setState({audio: new Audio(src)}, () => {
            console.log("Playing audio: " + Anchor.instance.state.audio.src);
            Anchor.instance.state.audio.volume = Anchor.instance.state.volume;
            Anchor.startAudio();
            Anchor.instance.state.audio.addEventListener("ended", () => {
                console.log("Playing audio: " + Anchor.instance.state.audio.src);
                Anchor.startAudio();
            });
        });
    }
    static startAudio() {
        let playPromise = Anchor.instance.state.audio.play();
        playPromise.then(() => {

            // Anchor.instance.state.audio.duration;
            // Anchor.instance.state.audio.currentTime = 45;
            // Anchor.instance.state.audio.playbackRate = 2;
            // if(Anchor.instance.state.audio.duration !== Infinity && !Number.isNaN(Anchor.instance.state.audio.duration)){
            //     let startTime = Math.ceil(Anchor.instance.state.audio.duration - (timeLeftSeconds ?? 0));
            //     if (startTime > 0 && startTime < Anchor.instance.state.audio.duration) {
            //         console.log("Starting audio at " + startTime + " seconds")
            //         Anchor.instance.state.audio.currentTime = startTime;
            //     };
            // }
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
            {this.state.settings_menu && <Settings 
                volume={this.state.audio.volume} 
                onVolumeChange={(volume) => {
                    this.state.audio.volume = volume
                    // this.setState({audio: this.state.audio});
                    this.forceUpdate();
                }}
            />}
            <button className="material-icons-round settings-button" onClick={() => {
                this.setState({settings_menu: !this.state.settings_menu});
            }}>settings</button>
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
