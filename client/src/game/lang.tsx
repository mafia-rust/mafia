import langJson from "../resources/lang.json";
import GAME_MANAGER from "../index";

let lang: Map<string, string> = new Map<string, string>(Object.entries(langJson));

export default function translate(langKey: string, ...valuesList: any[]): string {
    let out = lang.get(langKey);
    if(out===undefined){
        console.log("Error: Attempted to use non existant lang key: "+langKey);
        return langKey;
    }
    for(let i = 0; i < valuesList.length; i++){
        out = out.replace("\\"+(i), valuesList[i]);
    }
    return out;
}

// TODO, make message union type (& make an interface) & make getChatString a method
export function getChatString(message: any): string {
    if(message.Normal !== undefined){
        if(message.Normal.message_sender.Player !== undefined){
            let playerIndex = message.Normal.message_sender.Player;
            return translate("chatmessage.Normal", 
                GAME_MANAGER.getPlayer(playerIndex), 
                message.Normal.text
            );
        }else{
            //TODO
        }
    }
    if(message.Whisper !== undefined){
        return translate("chatmessage.Whisper", 
            GAME_MANAGER.getPlayer(message.Whisper.from_player_index), 
            GAME_MANAGER.getPlayer(message.Whisper.to_player_index),
            message.Whisper.text
        );
    }
    if(message.BroadcastWhisper!==undefined){
        return translate("chatmessage.BroadcastWhisper",
            GAME_MANAGER.getPlayer(message.BroadcastWhisper.whisperer),
            GAME_MANAGER.getPlayer(message.BroadcastWhisper.whisperee)
        );
    }
    if(message.RoleAssignment!==undefined){
        
        let role = null;
        let name = null
        let description = null;

        role = message.RoleAssignment.role

        name = lang.get("role."+role+".name");
        description = lang.get("role."+role+".description");

        return translate("chatmessage.RoleAssignment", name, description);
    }
    if(message.PlayerDied!==undefined){
        //TODO, role doesnt work properly
        return translate("chatmessage.PlayerDied",
            GAME_MANAGER.getPlayer(message.PlayerDied.grave.player),
            message.PlayerDied.grave.role,
            "UNINPLEMENTED",
            message.PlayerDied.grave.will
        );
    }
    if(message.PhaseChange!==undefined){
        return translate("chatmessage.PhaseChange",
            lang.get("phase."+message.PhaseChange.phase_type),
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
                GAME_MANAGER.getPlayer(message.Voted.voter)
            );
        }
        return translate("chatmessage.Voted",
            GAME_MANAGER.getPlayer(message.Voted.voter),
            GAME_MANAGER.getPlayer(message.Voted.votee),
        );
    }
    if(message.PlayerOnTrial!==undefined){
        return translate("chatmessage.PlayerOnTrial",
            GAME_MANAGER.getPlayer(message.PlayerOnTrial.player_index)
        );
    }
    if(message.JudgementVote!==undefined){
        return translate("chatmessage.JudgementVote",
            GAME_MANAGER.getPlayer(message.JudgementVote.voter_player_index)
        );
    }
    if(message.JudgementVerdict!==undefined){
        return translate("chatmessage.JudgementVerdict",
            GAME_MANAGER.getPlayer(message.JudgementVerdict.voter_player_index),
            lang.get("verdict."+message.JudgementVerdict.verdict)
        );
    }
    if(message.TrialVerdict!==undefined){
        return translate("chatmessage.TrialVerdict",
            message.TrialVerdict.innocent>=message.TrialVerdict.guilty?lang.get("verdict.Innocent"):lang.get("verdict.Guilty"),
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
                GAME_MANAGER.getPlayer(message.Targeted.targeter),
                GAME_MANAGER.getPlayer(message.Targeted.target)
            );
        }
        return translate("chatmessage.Targeted_null",
            GAME_MANAGER.getPlayer(message.Targeted.targeter)
        );
    }
    if(message.MayorRevealed !== undefined) {
        return translate("chatmessage.MayorRevealed",
            GAME_MANAGER.getPlayer(message.MayorRevealed.player_index)
        );
    }
    return translate("chatmessage."+message);
    // return JSON.stringify(message);
}

// TODO make night information message union type (& make an interface) and make this a method
export function getNightInformationString(message: any){

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
