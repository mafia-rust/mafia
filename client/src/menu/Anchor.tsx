import React from "react";
import "../index.css";
import "./anchor.css";
import GAME_MANAGER from "..";
import translate from "../game/lang";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    mobile: boolean,
    content: JSX.Element,
    error: JSX.Element | null,
    rejoinCard: JSX.Element | null
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        this.state = {
            mobile: false,
            content: this.props.content,
            error: null,
            rejoinCard: null
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

    render(){
        return <div className="anchor">
            {this.state.content}
            {this.state.error}
            {this.state.rejoinCard}
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
