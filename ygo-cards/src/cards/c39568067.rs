//! Fateful Adventure
use super::prelude::*;

pub const ID: u32 = 39568067;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent battle destruction once
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetValue(function(_,_,r) return r&REASON_BATTLE==REASON_BATTLE end)
    effect::set_value_fn(d, e2, lua_fn(indestructable_value));
    // e2:SetTarget(function(_,c) return c:GetEquipCount()>0 end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::get_equip_count(d, c) > 0)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search monster that lists "Adventurer Token"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.mthtg)
    effect::set_target(d, e3, Some(cost_target(mthtg)));
    // e3:SetOperation(s.mthop)
    effect::set_operation(d, e3, Some(operation(mthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Search Equip Spell that lists "Adventurer Token"
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_EQUIP)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_EQUIP);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.sthtg)
    effect::set_target(d, e4, Some(cost_target(sthtg)));
    // e4:SetOperation(s.sthop)
    effect::set_operation(d, e4, Some(operation(sthop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.mthfilter(c)`
fn mthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:ListsCode(TOKEN_ADVENTURER) and c:IsAbleToHand()
    card::is_monster(d, c) && card::lists_code(d, c, &[TOKEN_ADVENTURER]) && card::is_able_to_hand(d, c, None)
}
/// `s.mthtg(e,tp,...,chk)`
fn mthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.mthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&mthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 1);
    true
}
/// `s.mthop(e,tp,...)`
fn mthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.mthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&mthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT)
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT, NO_EX);
    }
}
/// `function(_,_,r) return r&REASON_BATTLE==REASON_BATTLE end` (the value of the battle protection)
fn indestructable_value(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2);
    // Lua: `&` binds tighter than `==`
    (r & REASON_BATTLE as i64) == REASON_BATTLE as i64
}
/// `s.adventurerfilter(c,ec)`
fn adventurerfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(TOKEN_ADVENTURER) and ec:CheckEquipTarget(c)
    card::is_faceup(d, c) && card::is_code(d, c, &[TOKEN_ADVENTURER]) && card::check_equip_target(d, ec, c)
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:CheckUniqueOnField(tp) and not c:IsForbidden() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.adventurerfilter,tp,LOCATION_MZONE,0,1,nil,c)
    card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_target(d, Some(&|d, x| adventurerfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sthfilter(c,tp)`
fn sthfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsEquipSpell() and c:ListsCode(TOKEN_ADVENTURER) and (c:IsAbleToHand() or s.eqfilter(c,tp))
    card::is_equip_spell(d, c) && card::lists_code(d, c, &[TOKEN_ADVENTURER]) && (card::is_able_to_hand(d, c, None) || eqfilter(d, c, tp))
}
/// `s.sthtg(e,tp,...,chk)`
fn sthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sthfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| sthfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.sthop(e,tp,...)`
fn sthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local eqc=Duel.SelectMatchingCard(tp,s.sthfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| sthfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not eqc then return end
    let Some(eqc) = group::get_first(d, &g) else {
        return;
    };
    // aux.ToHandOrElse(eqc,tp, function(eqc) return s.eqfilter(eqc,tp) end, function(eqc) ... end, aux.Stringid(id,2))
    aux::to_hand_or_else(
        d,
        eqc,
        tp,
        Some(&|d, c| eqfilter(d, c, tp)),
        Some(&|d, _g: &CardSet| {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
            // local g=Duel.SelectMatchingCard(tp,s.adventurerfilter,tp,LOCATION_MZONE,0,1,1,nil,eqc)
            let g = duel::select_matching_card(d, tp, Some(&|d, x| adventurerfilter(d, x, eqc)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // local tc=g:GetFirst()
            // if tc then Duel.Equip(tp,eqc,tc) end
            if let Some(tc) = group::get_first(d, &g) {
                duel::equip(d, tp, eqc, tc, true, false);
            }
            0
        }),
        Some(aux::stringid(ID, 2)),
    );
}
