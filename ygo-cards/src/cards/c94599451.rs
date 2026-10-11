//! Mythical Institution
use super::prelude::*;

pub const ID: u32 = 94599451;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place Spell counters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 monster to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destruction replacement
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // e4:SetOperation(s.desrepop)
    effect::set_operation(d, e4, Some(operation(desrepop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ctfilter(c,tp)`
fn ctfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_ONFIELD)
    //   and (c:GetPreviousTypeOnField()&TYPE_PENDULUM)~=0 and c:IsPreviousSetCard(SET_MYTHICAL_BEAST)
    //   and c:IsPreviousControler(tp) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && (card::get_previous_type_on_field(d, c) & TYPE_PENDULUM) != 0
        && card::is_previous_set_card(d, c, &[SET_MYTHICAL_BEAST as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.ctfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| ctfilter(d, c, tp), 1, NO_EX)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(COUNTER_SPELL,2)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, COUNTER_SPELL as u16, 2, false);
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) and c:IsLevelAbove(1)
    //   and c:IsAbleToHand() and (c:IsFaceup() or c:IsLocation(LOCATION_DECK))
    //   and Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,c:GetLevel(),REASON_COST)
    card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
        && card::is_level_above(d, c, 1)
        && card::is_able_to_hand(d, c, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
        && {
            let lv = card::get_level(d, c);
            duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, lv, REASON_COST)
        }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
    // lvt: the distinct levels of g, compacted into 1..n in ascending order (Lua table code)
    let mut present = [false; 13];
    for tc in group::iter(d, &g) {
        let tlv = card::get_level(d, tc) as usize;
        if (1..=12).contains(&tlv) {
            present[tlv] = true;
        }
    }
    let levels: Vec<u64> = (1..=12).filter(|&i| present[i]).map(|i| i as u64).collect();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local lv=Duel.AnnounceNumber(tp,table.unpack(lvt))
    let lv = duel::announce_number(d, tp, &levels).nil().0;
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,lv,REASON_COST)
    duel::remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, lv as u32, REASON_COST);
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) and c:IsLevel(lv)
    //   and c:IsAbleToHand() and (c:IsFaceup() or c:IsLocation(LOCATION_DECK))
    card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
        && card::is_level(d, c, &[lv])
        && card::is_able_to_hand(d, c, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,lv)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, lv)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not e:GetHandler():IsReason(REASON_RULE)
    //   and e:GetHandler():GetCounter(COUNTER_SPELL)>0 end
    if a.chk == Some(0) {
        return !card::is_reason(d, h, REASON_RULE) && card::get_counter(d, h, COUNTER_SPELL as u16) > 0;
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(ep,COUNTER_SPELL,1,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, h, a.ep, COUNTER_SPELL as u16, 1, REASON_EFFECT);
}
