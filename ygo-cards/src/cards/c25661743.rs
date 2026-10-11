//! Prohibited Power Patron Portal - Terminus
use super::prelude::*;

pub const ID: u32 = 25661743;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POWER_PATRON as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "Power Patron" monster from your Deck or Extra Deck to the GY, then you can add 1 DARK Fairy monster from your Deck to your hand, also you cannot declare attacks for the rest of this turn, except with "Power Patron" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // At the start of the Damage Step, if your "Power Patron" monster battles an opponent's monster: You can banish this card from your GY; banish (face-down) that opponent's monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.bancon)
    effect::set_condition(d, e2, Some(condition(bancon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.bantg)
    effect::set_target(d, e2, Some(cost_target(bantg)));
    // e2:SetOperation(s.banop)
    effect::set_operation(d, e2, Some(operation(banop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_POWER_PATRON) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_FAIRY) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_FAIRY) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE)
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) > 0
            && card::is_location(d, sc, LOCATION_GRAVE)
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
            let sg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - tp, sg);
            }
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // --You cannot declare attacks for the rest of this turn, except with "Power Patron" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_POWER_PATRON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_POWER_PATRON as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.bancon(e,tp,...)`
fn bancon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,b=Duel.GetBattleMonster(tp)
    let (ma, mb) = duel::get_battle_monster(d, a.tp);
    // return a and b and a:IsSetCard(SET_POWER_PATRON) and a:IsFaceup() and b:IsAbleToRemove(nil,POS_FACEDOWN)
    let (Some(ma), Some(mb)) = (ma, mb) else {
        return false;
    };
    card::is_set_card(d, ma, &[SET_POWER_PATRON as u16]) && card::is_faceup(d, ma) && card::is_able_to_remove(d, mb, None, Some(POS_FACEDOWN as u8), None)
}
/// `s.bantg(e,tp,...,chk)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local _,b=Duel.GetBattleMonster(tp)
    let (_, b) = duel::get_battle_monster(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,b,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, b.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.banop(e,tp,...)`
fn banop(d: &mut Duel, a: &EffArgs) {
    // local _,b=Duel.GetBattleMonster(tp)
    let (_, b) = duel::get_battle_monster(d, a.tp);
    // if b and b:IsRelateToBattle() and b:IsControler(1-tp) then
    if let Some(b) = b {
        if card::is_relate_to_battle(d, b) && card::is_controler(d, b, 1 - a.tp) {
            // Duel.Remove(b,POS_FACEDOWN,REASON_EFFECT)
            duel::remove(d, b, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        }
    }
}
