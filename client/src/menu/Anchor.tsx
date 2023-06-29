import React from "react";
import "../index.css";
import "./anchor.css";

type AnchorProps = {
    content: JSX.Element,
    onMount: () => void
}
type AnchorState = {
    mobile: boolean,
    content: JSX.Element,
    info: JSX.Element | null
}

export default class Anchor extends React.Component<AnchorProps, AnchorState> {
    private static instance: Anchor;

    constructor(props: AnchorProps) {
        super(props);

        this.state = {
            mobile: false,
            content: this.props.content,
            info: null
        }
    }
    
    componentDidMount() {
        Anchor.instance = this;

        window.addEventListener("resize", Anchor.onResize);
        Anchor.onResize();

        this.props.onMount()
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
    
    componentWillUnmount() {
        window.removeEventListener("resize", Anchor.onResize);
    }

    render(){
        return <div className="anchor">
            {this.state.content}
            {this.state.info}
        </div>
    }

    public static setContent(content: JSX.Element){
        Anchor.instance.setState({content : content});
    }
    public static pushInfo(title: string, body: string) {
        Anchor.instance.setState({info: <ErrorCard
            onClose={() => Anchor.instance.setState({ info: null })}
            error={{title, body}}
        />});
    }
    public static isMobile(): boolean {
        return Anchor.instance.state.mobile;
    }
}

interface Error {
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
