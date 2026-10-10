//! Destiny Board
use super::prelude::*;

pub const ID: u32 = 94212438;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SPIRIT_MESSAGE], ..ScriptInfo::DEFAULT };

/// `CARDS_SPIRIT_MESSAGE` (card_counter_constants.lua), 1-based in Lua
const CARDS_SPIRIT_MESSAGE: [u32; 4] = [31893528, 67287533, 94772232, 30170981];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // place card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_label(d, e2, &[ID as i64]);
    effect::set_condition(d, e2, Some(condition(plcon)));
    effect::set_operation(d, e2, Some(operation(plop)));
    // e2:SetValue(s.extraop)
    effect::set_value_fn(d, e2, operation(extraop));
    card::register_effect(d, c, e2, false);
    // tograve
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e3, Some(condition(tgcon)));
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e4, Some(operation(tgop)));
    card::register_effect(d, c, e4, false);
    // win
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e5, EVENT_ADJUST);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_property(d, e5, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e5, Some(operation(winop)));
    card::register_effect(d, c, e5, false);
}
/// `s.plcon(e,...)`
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetFlagEffect(id)<4
    duel::is_turn_player(d, 1 - a.tp) && {
        let h = effect::get_handler(d, a.e).nil();
        card::get_flag_effect(d, h, ID) < 4
    }
}
/// `s.plfilter(c,code)`
fn plfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and not c:IsForbidden()
    card::is_code(d, c, &[code]) && !card::is_forbidden(d, c)
}
/// `s.plop(e,...)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_DARK_SANCTUARY) then return s.extraop(e,tp,eg,ep,ev,re,r,rp) end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_DARK_SANCTUARY).is_some() {
        extraop(d, a);
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local passcode=CARDS_SPIRIT_MESSAGE[c:GetFlagEffect(id)+1]
    let passcode = CARDS_SPIRIT_MESSAGE.get(card::get_flag_effect(d, c, ID)).copied().nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,passcode)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| plfilter(d, c, passcode)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.MoveToField(g:GetFirst(),tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        if duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        }
    }
}
/// `s.extraop(e,...)`
fn extraop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local cid=CARDS_SPIRIT_MESSAGE[c:GetFlagEffect(id)+1]
    let cid = CARDS_SPIRIT_MESSAGE.get(card::get_flag_effect(d, c, ID)).copied().nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,cid)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| plfilter(d, cc, cid)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.IsPlayerCanSpecialSummonMonster(tp,cid,0,TYPE_MONSTER|TYPE_NORMAL,0,0,1,RACE_FIEND,ATTRIBUTE_DARK,POS_FACEUP,tp,181)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and (Duel.GetLocationCount(tp,LOCATION_SZONE)<1 or Duel.SelectYesNo(tp,aux.Stringid(CARD_DARK_SANCTUARY,0))) then
    if let Some(tc) = tc {
        if duel::is_player_can_special_summon_monster(
            d,
            a.tp,
            cid,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPE_MONSTER | TYPE_NORMAL),
                attack: Some(0),
                defense: Some(0),
                level: Some(1),
                race: Some(RACE_FIEND),
                attribute: Some(ATTRIBUTE_DARK),
            },
            Some(POS_FACEUP as u8),
            Some(a.tp),
            181,
        ) && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && (duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 < 1
                || duel::select_yes_no(d, a.tp, aux::stringid(CARD_DARK_SANCTUARY, 0)))
        {
            // tc:AddMonsterAttribute(TYPE_NORMAL,ATTRIBUTE_DARK,RACE_FIEND,1,0,0)
            card::add_monster_attribute(d, tc, TYPE_NORMAL, ATTRIBUTE_DARK, RACE_FIEND, 1, 0, 0);
            // Duel.SpecialSummonStep(tc,181,tp,tp,true,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 181, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
            // tc:AddMonsterAttributeComplete()
            card::add_monster_attribute_complete(d, tc);
            // immune
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_value_fn(d, e1, lua_fn(immune_value));
            effect::set_reset(d, e1, RESET_EVENT | RESET_TODECK | RESET_TOHAND | RESET_TOGRAVE | RESET_REMOVE, 1);
            card::register_effect(d, tc, e1, false);
            // cannot be target
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_IGNORE_BATTLE_TARGET);
            effect::set_value(d, e2, 1);
            effect::set_reset(d, e2, RESET_EVENT | RESET_TODECK | RESET_TOHAND | RESET_TOGRAVE | RESET_REMOVE, 1);
            card::register_effect(d, tc, e2, false);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        } else if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        }
    }
}
/// `function(e,te) return not te:IsCardCode(id) end` (the immune value)
fn immune_value(d: &mut Duel, a: Args) -> bool {
    !effect::is_card_code(d, a.e(1), &[ID])
}
/// `s.cfilter1(c,tp)`
fn cfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and (c:IsCode(id) or c:IsSetCard(SET_SPIRIT_MESSAGE))
    card::is_controler(d, c, tp) && (card::is_code(d, c, &[ID]) || card::is_set_card(d, c, &[SET_SPIRIT_MESSAGE as u16]))
}
/// `s.cfilter2(c)`
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(id) or c:IsSetCard(SET_SPIRIT_MESSAGE))
    card::is_faceup(d, c) && (card::is_code(d, c, &[ID]) || card::is_set_card(d, c, &[SET_SPIRIT_MESSAGE as u16]))
}
/// `s.tgcon(e,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter1,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter1(d, c, a.tp), 1, NO_EX)
}
/// `s.tgop(e,...)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.cfilter2,tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter2), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
/// `s.cfilter3(c)`
fn cfilter3(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(table.unpack(CARDS_SPIRIT_MESSAGE))
    card::is_faceup(d, c) && card::is_code(d, c, &CARDS_SPIRIT_MESSAGE)
}
/// `s.winop(e,...)`
fn winop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.cfilter3,tp,LOCATION_ONFIELD,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&cfilter3), a.tp, LOCATION_ONFIELD, 0, h.into());
    // if g:GetClassCount(Card.GetCode)==4 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) == 4 {
        // Duel.Win(tp,WIN_REASON_DESTINY_BOARD)
        duel::win(d, a.tp, WIN_REASON_DESTINY_BOARD);
    }
}
