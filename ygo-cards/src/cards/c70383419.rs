//! Invoked Baybarron
//! 召喚獣ベイバロン
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 70383419;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[10673071, 86319972],
    listed_series: &[SET_ALEISTER],
    material_setcode: &[SET_ALEISTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Aleister" monster + 1 LIGHT or EARTH monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ALEISTER),aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT|ATTRIBUTE_EARTH))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ALEISTER as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, (ATTRIBUTE_LIGHT | ATTRIBUTE_EARTH) as u32, Some(fc), st, Some(tp)))),
        ],
    );
    // If this card is Special Summoned: You can add 1 "Virakam the Artificial Spirit" or "Magical Name - "Rosa Mundi"" from your Deck to your hand, then you can banish 1 monster from either GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY, then target 1 Fusion Monster you control; all monsters your opponent currently controls lose ATK equal to that monster's until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(10673071,86319972) and c:IsAbleToHand()
    card::is_code(d, c, &[10673071, 86319972]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
// aux.AND(Card.IsMonster,Card.IsAbleToRemove)
fn monster_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // if Duel.IsExistingMatchingCard(aux.AND(Card.IsMonster,Card.IsAbleToRemove),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&monster_removable), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
            // local rg=Duel.SelectMatchingCard(tp,aux.AND(Card.IsMonster,Card.IsAbleToRemove),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
            let rg = duel::select_matching_card(d, a.tp, Some(&monster_removable), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
            // if #rg>0 then
            if !rg.is_empty() {
                // Duel.HintSelection(rg)
                duel::hint_selection(d, &rg, true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
// aux.AND(Card.IsFusionMonster,Card.HasNonZeroAttack)
fn fusion_nonzero_attack(d: &mut Duel, c: CardId) -> bool {
    card::is_fusion_monster(d, c) && card::has_non_zero_attack(d, c)
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFusionMonster() and chkc:HasNonZeroAttack() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_fusion_monster(d, chkc)
            && card::has_non_zero_attack(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsFusionMonster,Card.HasNonZeroAttack),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&fusion_nonzero_attack), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsFusionMonster,Card.HasNonZeroAttack),tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&fusion_nonzero_attack), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,nil,1,tp,g:GetFirst():GetAttack())
    let first = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, a.tp, atk);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // local atk=tc:GetAttack()
    let atk = card::get_attack(d, tc) as i64;
    // if atk<=0 then return end
    if atk <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for oc in g:Iter() do
    for oc in group::iter(d, &g) {
        // --All monsters your opponent currently controls lose ATK equal to that monster's until the end of this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-atk)
        effect::set_value(d, e1, -atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // oc:RegisterEffect(e1)
        card::register_effect(d, oc, e1, false);
    }
}
