import React from "react";
import { getChatString } from "../../game/lang";
import GAME_MANAGER from "../../index";
import "./gameScreen.css";
import "./chatMenu.css"

export default class ChatMenu extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      gameState: GAME_MANAGER.gameState,
      chatField: "",
    };

    this.listener = () => {
      this.setState({
        gameState: GAME_MANAGER.gameState
      });
    };
  }

  componentDidMount() {
    GAME_MANAGER.addStateListener(this.listener);
  }

  componentWillUnmount() {
    GAME_MANAGER.removeStateListener(this.listener);
  }

  handleInputChange = (event) => {
    const value = event.target.value.trimStart();
    this.setState({
      chatField: value
    });
  };

  handleInputKeyPress = (event) => {
    if (event.code === "Enter") {
      event.preventDefault();
      this.sendChatField();
    }
  };

  sendChatField = () => {
    const text = this.state.chatField.trim();
    if (text.startsWith("/w")) {
      try {
        const playerIndex = Number(text[2]) - 1;
        const message = text.substring(3);
        GAME_MANAGER.sendWhisper_button(playerIndex, message);
      } catch (e) {
        GAME_MANAGER.sendMessage_button(text);
      }
    } else {
      GAME_MANAGER.sendMessage_button(text);
    }
    this.setState({
      chatField: ""
    });
  };

  calcInputHeight = (value) => {
    const numberOfLineBreaks = (value.match(/\n/g) || []).length;
    // min-height + lines x line-height + padding + border
    const newHeight = 20 + numberOfLineBreaks * 20 + 12 + 2;
    return newHeight;
  };

  renderTextInput() {
    return (
      <div className="chat-input-container">
        <textarea
          className="chat-input"
          value={this.state.chatField}
          onChange={this.handleInputChange}
          onKeyPress={this.handleInputKeyPress}
          style={{ height: this.calcInputHeight(this.state.chatField) }}
        />
        <button
          className="gm-button"
          onClick={this.sendChatField}
        >
          Send
        </button>
      </div>
    );
  }

  renderChatMessage(msg, i) {
    return (
      <div key={i} className="chat-message">
        {getChatString(msg)}
      </div>
    );
  }

  render() {
    return (
      <div className="chat-menu">
        <div className="chat-messages">
          {this.state.gameState.chatMessages.map((msg, i) => {
            return this.renderChatMessage(msg, i);
          })}
        </div>
        {this.renderTextInput()}
      </div>
    );
  }
}
