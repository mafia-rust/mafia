import lang from "../resources/lang.json";
import gameManager from "..";

export function translate(langKey, ...valuesList){
    let out = lang[langKey];
    if(out===undefined||out===null){
        console.log("Error: Attempted to use non existant lang key: "+langKey);
        return langKey;
    }
    for(let i = 0; i < valuesList.length; i++){
        out = out.replace("\\"+(i), valuesList[i]);
    }
    return out;
}

export function getPlayerString(playerIndex){
    if(gameManager.gameState.players[playerIndex] === undefined){
        return "";
    }
    return "("+(playerIndex+1)+") "+
    gameManager.gameState.players[playerIndex].name;
}

export function getChatString(message) {
    if(message.Normal !== undefined){
        if(message.Normal.message_sender.Player !== undefined){
            return translate("chatmessage.Normal", 
                getPlayerString(message.Normal.message_sender.Player), 
                message.Normal.text
            );
        }else{
            //TODO
        }
    }
    if(message.Whisper !== undefined){
        return translate("chatmessage.Whisper", 
            getPlayerString(message.Whisper.from_player_index), 
            getPlayerString(message.Whisper.to_player_index),
            message.Whisper.text
        );
    }
    if(message.BroadcastWhisper!==undefined){
        return translate("chatmessage.BroadcastWhisper",
            getPlayerString(message.BroadcastWhisper.whisperer),
            getPlayerString(message.BroadcastWhisper.whisperee)
        );
    }
    if(message.RoleAssignment!==undefined){
        
        let role = null;
        let name = null
        let description = null;

        role = message.RoleAssignment.role

        name = lang["role."+role+".name"];
        description = lang["role."+role+".description"];

        return translate("chatmessage.RoleAssignment", name, description);
    }
    if(message.PlayerDied!==undefined){
        //TODO, role doesnt work properly
        return translate("chatmessage.PlayerDied",
            getPlayerString(message.PlayerDied.grave.player),
            message.PlayerDied.grave.role,
            "UNINPLEMENTED",
            message.PlayerDied.grave.will
        );
    }
    if(message.PhaseChange!==undefined){
        return translate("chatmessage.PhaseChange",
            lang["phase."+message.PhaseChange.phase_type],
            message.PhaseChange.day_number
        );
    }
    if(message.TrialInformation!==undefined){
        return translate("chatmessage.TrialInformation",
            message.TrialInformation.required_votes,
            message.TrialInformation.trials_left
        );
    }
    if(message.Voted!==undefined){
        if(message.Voted.votee==null){
            return translate("chatmessage.Voted_null",
                getPlayerString(message.Voted.voter),
            );
        }
        return translate("chatmessage.Voted",
            getPlayerString(message.Voted.voter),
            getPlayerString(message.Voted.votee),
        );
    }
    if(message.PlayerOnTrial!==undefined){
        return translate("chatmessage.PlayerOnTrial",
            getPlayerString(message.PlayerOnTrial.player_index)
        );
    }
    if(message.JudgementVote!==undefined){
        return translate("chatmessage.JudgementVote",
            getPlayerString(message.JudgementVote.voter_player_index)
        );
    }
    if(message.JudgementVerdict!==undefined){
        return translate("chatmessage.JudgementVerdict",
            getPlayerString(message.JudgementVerdict.voter_player_index),
            lang["verdict."+message.JudgementVerdict.verdict]
        );
    }
    if(message.TrialVerdict!==undefined){
        return translate("chatmessage.TrialVerdict",
            message.TrialVerdict.innocent>=message.TrialVerdict.guilty?lang["verdict.Innocent"]:lang["verdict.Guilty"],
            message.TrialVerdict.innocent,
            message.TrialVerdict.guilty
        );
    }
    if(message.NightInformation!==undefined){
        return getNightInformationString(message.NightInformation.night_information);
    }
    if(message.Targeted!==undefined){
        if(message.Targeted.target!==null){
            return translate("chatmessage.Targeted",
                getPlayerString(message.Targeted.targeter),
                getPlayerString(message.Targeted.target)
            );
        }
        return translate("chatmessage.Targeted_null",
            getPlayerString(message.Targeted.targeter),
        );
    }
    if(message.MayorRevealed !== undefined) {
        return translate("chatmessage.MayorRevealed",
            getPlayerString(message.MayorRevealed.player_index)
        );
    }
    return translate("chatmessage."+message);
    // return JSON.stringify(message);
}
export function getNightInformationString(message){

    if (message.RoleBlocked !== undefined) {
        if (message.RoleBlocked.immune) {
            return translate("chatmessage.night.RoleBlocked_immune");
        }
        return translate("chatmessage.night.RoleBlocked");
    }
    if (message.SheriffResult !== undefined) {
        if(message.SheriffResult.suspicious)
            return translate("chatmessage.night.SheriffResult_suspicious");
        return translate("chatmessage.night.SheriffResult_innocent");
    }

    return translate("chatmessage.night."+message);
}
