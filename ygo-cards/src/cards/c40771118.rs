//! Sentence of Doom
use super::prelude::*;

pub const ID: u32 = 40771118;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DESTINY_BOARD], listed_series: &[SET_SPIRIT_MESSAGE], ..ScriptInfo::DEFAULT };

/// `CARDS_SPIRIT_MESSAGE` (card_counter_constants.lua), 1-based in Lua
const CARDS_SPIRIT_MESSAGE: [u32; 4] = [31893528, 67287533, 94772232, 30170981];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // add to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // place on the field
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_label(d, e3, &[CARD_DESTINY_BOARD as i64]);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(plcost)));
    effect::set_target(d, e3, Some(cost_target(pltg)));
    effect::set_operation(d, e3, Some(operation(plop)));
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsCode(CARD_DESTINY_BOARD) or c:IsSetCard(SET_SPIRIT_MESSAGE))
    card::is_faceup(d, c) && (card::is_code(d, c, &[CARD_DESTINY_BOARD]) || card::is_set_card(d, c, &[SET_SPIRIT_MESSAGE as u16]))
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsMonster() and c:IsAbleToHand() and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_race(d, c, RACE_FIEND) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_ONFIELD, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    }
}
/// `s.smfilter(c)`
fn smfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(table.unpack(CARDS_SPIRIT_MESSAGE)) and not c:IsForbidden()
    card::is_code(d, c, &CARDS_SPIRIT_MESSAGE) && !card::is_forbidden(d, c)
}
/// `s.plcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn plcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.smfilter,tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&smfilter), tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_DARK_SANCTUARY) then return s.extraop(e,tp,eg,ep,ev,re,r,rp) end
    if duel::is_player_affected_by_effect(d, tp, CARD_DARK_SANCTUARY).is_some() {
        return extraop(d, a);
    }
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.smfilter),tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(smfilter)), tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.MoveToField(g:GetFirst(),tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        let tc = group::get_first(d, &g).nil();
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.extraop(e,tp,eg,ep,ev,re,r,rp)`
fn extraop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.smfilter),tp,LOCATION_DECK|LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(smfilter)), tp, LOCATION_DECK | LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // if Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_NORMAL,0,0,1,RACE_FIEND,ATTRIBUTE_DARK,POS_FACEUP,tp,181)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and (Duel.GetLocationCount(tp,LOCATION_SZONE)<1 or Duel.SelectYesNo(tp,aux.Stringid(CARD_DARK_SANCTUARY,0))) then
    if duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID,
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
        Some(tp),
        181,
    ) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && (duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 1
            || duel::select_yes_no(d, tp, aux::stringid(CARD_DARK_SANCTUARY, 0)))
    {
        // tc:AddMonsterAttribute(TYPE_NORMAL,ATTRIBUTE_DARK,RACE_FIEND,1,0,0)
        card::add_monster_attribute(d, tc, TYPE_NORMAL, ATTRIBUTE_DARK, RACE_FIEND, 1, 0, 0);
        // Duel.SpecialSummonStep(tc,181,tp,tp,true,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 181, tp, tp, true, false, POS_FACEUP as u8, None);
        // tc:AddMonsterAttributeComplete()
        card::add_monster_attribute_complete(d, tc);
        // immune
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_value_fn(d, e1, lua_fn(efilter));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        card::register_effect(d, tc, e1, false);
        // cannot be target
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_IGNORE_BATTLE_TARGET);
        effect::set_value(d, e2, 1);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
        card::register_effect(d, tc, e2, false);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local tc=te:GetHandler()
    let te = a.e(1);
    let tc = effect::get_handler(d, te).nil();
    // return not tc:IsCode(CARD_DESTINY_BOARD)
    !card::is_code(d, tc, &[CARD_DESTINY_BOARD])
}
