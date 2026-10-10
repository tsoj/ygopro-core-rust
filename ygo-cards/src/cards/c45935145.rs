//! Wollow, Founder of the Drudge Dragons
use super::prelude::*;

pub const ID: u32 = 45935145;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Monsters you control gain 100 ATK/DEF for each card in your opponent's GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_GRAVE)*100 end)
    effect::set_value_fn(d, e1, lua_fn(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Activate the appropriate effect based on the number of detached materials
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // s.effdetachcost=Cost.Choice(
    //     {Cost.DetachFromSelf(1),aux.Stringid(id,1),s.tccheck(1)},
    //     {Cost.DetachFromSelf(2),aux.Stringid(id,2),s.tccheck(2)})
    let effdetachcost = cost::choice(vec![
        cost::CostChoice {
            cost: cost::detach_from_self(cost::Count::N(1), None, None),
            desc: aux::stringid(ID, 1),
            check: Some(lua_fn(|d, a| tccheck(d, a.e(0), a.p(1), 1))),
        },
        cost::CostChoice {
            cost: cost::detach_from_self(cost::Count::N(2), None, None),
            desc: aux::stringid(ID, 2),
            check: Some(lua_fn(|d, a| tccheck(d, a.e(0), a.p(1), 2))),
        },
    ]);
    // e3:SetCost(Cost.AND(s.efftgcost,s.effdetachcost))
    effect::set_cost(d, e3, Some(cost::and(vec![cost_target(efftgcost), effdetachcost])));
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_GRAVE)*100 end`
fn atk_value(d: &mut Duel, a: Args) -> i64 {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, 0, LOCATION_GRAVE) as i64 * 100
}
/// `s.tgfilter(c,e,tp,detach_1,detach_2)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, detach_1: bool, detach_2: bool) -> bool {
    // return (detach_1 and c:IsAbleToDeck()) or (detach_2 and s.setfilter(c,e,tp))
    (detach_1 && card::is_able_to_deck(d, c)) || (detach_2 && setfilter(d, c, e, tp))
}
/// `s.setfilter(c,e,tp)`
fn setfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsSpellTrap() then return c:IsSSetable() end
    if card::is_spell_trap(d, c) {
        return card::is_s_setable(d, c, false, None);
    }
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP|POS_FACEDOWN_DEFENSE)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
/// `s.efftgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local detach_1=c:CheckRemoveOverlayCard(tp,1,REASON_COST)
    let detach_1 = card::check_remove_overlay_card(d, c, a.tp, 1, REASON_COST);
    // local detach_2=c:CheckRemoveOverlayCard(tp,2,REASON_COST)
    let detach_2 = card::check_remove_overlay_card(d, c, a.tp, 2, REASON_COST);
    // if chk==0 then return (detach_1 or detach_2)
    //     and Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp,detach_1,detach_2) end
    if a.chk == Some(0) {
        return (detach_1 || detach_2)
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.e, a.tp, detach_1, detach_2)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp,detach_1,detach_2)
    let _ = duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, a.e, a.tp, detach_1, detach_2)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    true
}
/// `s.tccheck(op)`: `function(e,tp) ... end`
fn tccheck(d: &mut Duel, e: EffectId, tp: u8, op: u32) -> bool {
    // local tc=Duel.GetFirstTarget()
    // if not tc then return true end
    let Some(tc) = duel::first_target(d) else {
        return true;
    };
    // return op==1 and tc:IsAbleToDeck() or s.setfilter(tc,e,tp)
    (op == 1 && card::is_able_to_deck(d, tc)) || setfilter(d, tc, e, tp)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local cd=e:GetChainData()
    // local tc=Duel.GetFirstTarget()
    let cost_choice = chain::effect_data(d, a.e, None, chain::field::COST_CHOICE).int();
    let tc = duel::first_target(d);
    // if cd.cost_choice==1 then
    if cost_choice == 1 {
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, tc.map(CardSet::from_card), 1, a.tp, 0);
    } else if cost_choice == 2 {
        // local categ=CATEGORY_SET|CATEGORY_LEAVE_GRAVE
        let categ = CATEGORY_SET | CATEGORY_LEAVE_GRAVE;
        // if tc:IsMonster() then
        let tc = tc.nil();
        if card::is_monster(d, tc) {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON|categ)
            effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON | categ);
            // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, a.tp, 0);
        } else {
            // e:SetCategory(categ)
            effect::set_category(d, a.e, categ);
        }
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local cd=e:GetChainData()
    let cost_choice = chain::effect_data(d, a.e, None, chain::field::COST_CHOICE).int();
    // if cd.cost_choice==1 then
    if cost_choice == 1 {
        // Return it to the Deck
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    } else if cost_choice == 2 {
        // If the target is a monster, Special Summon it face-up, or in face-down Defense Position, to your field
        // if tc:IsMonster() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP|POS_FACEDOWN_DEFENSE)>0
        //     and tc:IsFacedown() then
        if card::is_monster(d, tc)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, (POS_FACEUP | POS_FACEDOWN_DEFENSE) as u8, None) > 0
            && card::is_facedown(d, tc)
        {
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - a.tp, tc);
        } else if card::is_s_setable(d, tc, false, None) {
            // If it is not, Set it on your field
            // Duel.SSet(tp,tc)
            duel::s_set(d, a.tp, tc, None, true);
        }
    }
}
