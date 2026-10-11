//! Witness of the Ancient
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 54577949;
/// `local TOKEN_ARC=id+1`
const TOKEN_ARC: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ARC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you have a Synchro Monster in your field or GY: You can Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned: You can place up to 3 Synchro Monsters with different names from your Extra Deck and/or GY in your Spell & Trap Zone as face-up Continuous Spells, and Special Summon 1 "Arc Token" (Machine/LIGHT/ATK 0/DEF 0) with a Level equal to the number placed, also for the rest of this turn, you cannot Special Summon from the Extra Deck, except Synchro Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSynchroMonster),tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_synchro_monster(d, c))), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSynchroMonster() and not c:IsForbidden()
    card::is_synchro_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.rescon(tokenlv1chk,tokenlv2chk,tokenlv3chk,mzone_chk)`: the `rescon` of the selection
fn rescon(lv1: bool, lv2: bool, lv3: bool, mzone_chk: bool) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        let n = sg.len();
        // if not mzone_chk then return sg:GetClassCount(Card.GetCode)==#sg end
        if !mzone_chk {
            return (group::get_class_count(d, sg, &|d, c| code_value(d, c)) == n, false);
        }
        // return (#sg==1 and tokenlv1chk) or (#sg==2 and tokenlv2chk) or (#sg==3 and tokenlv3chk), sg:GetClassCount(Card.GetCode)~=#sg
        let res = (n == 1 && lv1) || (n == 2 && lv2) || (n == 3 && lv3);
        (res, group::get_class_count(d, sg, &|d, c| code_value(d, c)) != n)
    }
}
/// `Card.GetCode` as one number (for the class counts)
fn code_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_code(d, c)[0] as i64
}
/// The Arc Token of the given Level (`Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ARC,0,TYPES_TOKEN,0,0,lv,RACE_MACHINE,ATTRIBUTE_LIGHT)`)
fn token_data(lv: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(lv),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_ARC,0,TYPES_TOKEN,0,0,lv,RACE_MACHINE,ATTRIBUTE_LIGHT)`
fn token_ok(d: &mut Duel, tp: u8, lv: u32) -> bool {
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_ARC, token_data(lv), None, None, 0)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local stzone_count=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let stzone_count = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // if stzone_count<=0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if stzone_count <= 0 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&plfilter), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, NO_EX);
        // if #g==0 then return false end
        if g.is_empty() {
            return false;
        }
        // local tokenlv1chk=stzone_count>=1 and Duel.IsPlayerCanSpecialSummonMonster(...)
        let tokenlv1chk = stzone_count >= 1 && token_ok(d, tp, 1);
        // local tokenlv2chk=stzone_count>=2 and Duel.IsPlayerCanSpecialSummonMonster(...)
        let tokenlv2chk = stzone_count >= 2 && token_ok(d, tp, 2);
        // local tokenlv3chk=stzone_count>=3 and Duel.IsPlayerCanSpecialSummonMonster(...)
        let tokenlv3chk = stzone_count >= 3 && token_ok(d, tp, 3);
        // return aux.SelectUnselectGroup(g,e,tp,1,math.min(3,stzone_count),s.rescon(tokenlv1chk,tokenlv2chk,tokenlv3chk,true),0)
        let rc = rescon(tokenlv1chk, tokenlv2chk, tokenlv3chk, true);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(3.min(stzone_count) as usize), Some(&rc), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local stzone_count=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let stzone_count = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.plfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(plfilter)), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, NO_EX);
    // if stzone_count>0 and #g>0 then
    if stzone_count > 0 && !g.is_empty() {
        // local place_success_chk=false
        let mut place_success_chk = false;
        // local mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mzone_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // local tokenlv1chk=stzone_count>=1 and Duel.IsPlayerCanSpecialSummonMonster(...)
        let tokenlv1chk = stzone_count >= 1 && token_ok(d, tp, 1);
        // local tokenlv2chk=stzone_count>=2 and Duel.IsPlayerCanSpecialSummonMonster(...)
        let tokenlv2chk = stzone_count >= 2 && token_ok(d, tp, 2);
        // local tokenlv3chk=stzone_count>=3 and Duel.IsPlayerCanSpecialSummonMonster(...)
        let tokenlv3chk = stzone_count >= 3 && token_ok(d, tp, 3);
        // local rescon=s.rescon(tokenlv1chk,tokenlv2chk,tokenlv3chk,mzone_chk)
        let rc = rescon(tokenlv1chk, tokenlv2chk, tokenlv3chk, mzone_chk);
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,math.min(3,#g,stzone_count),rescon,1,tp,HINTMSG_TOFIELD,rescon)
        let max = 3.min(g.len()).min(stzone_count as usize);
        let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(max), Some(&rc), 1, tp, Some(HINTMSG_TOFIELD as u64), Some(&rc), None, false).unwrap_or_default();
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // if Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
            if duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
                // place_success_chk=true
                place_success_chk = true;
                // --Treated as a Continuous Spell
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetCode(EFFECT_CHANGE_TYPE)
                effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
                effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
        // if place_success_chk and mzone_chk then
        if place_success_chk && mzone_chk {
            // local token=Duel.CreateToken(tp,TOKEN_ARC)
            let token = duel::create_token(d, tp, TOKEN_ARC).nil();
            // token:Level(#sg)
            card::info_level(d, token, Some(sg.len() as u32));
            // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // --For the rest of this turn, you cannot Special Summon from the Extra Deck, except Synchro Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSynchroMonster() end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, cc| card::is_location(d, cc, LOCATION_EXTRA) && !card::is_synchro_monster(d, cc))));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
