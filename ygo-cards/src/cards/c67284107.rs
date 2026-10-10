//! Scapeghost
use super::prelude::*;

pub const ID: u32 = 67284107;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[67284108], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn token_data() -> duel::MonsterData {
    // TYPES_TOKEN, 0, 0, level 1, RACE_ZOMBIE, ATTRIBUTE_DARK
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_ZOMBIE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_ZOMBIE,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=5
    let mut ft: i32 = 5;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // ft=math.min(ft,Duel.GetLocationCount(tp,LOCATION_MZONE))
    ft = ft.min(duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0);
    // if ft<=0 or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if ft <= 0 || !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // local i=0
    let mut i: u32 = 0;
    // repeat
    loop {
        // local token=Duel.CreateToken(tp,id+1+i)
        let token = duel::create_token(d, tp, ID + 1 + i).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // ft=ft-1
        ft -= 1;
        // i=(i+1)%4
        i = (i + 1) % 4;
        // until ft<=0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1))
        if ft <= 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            break;
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
