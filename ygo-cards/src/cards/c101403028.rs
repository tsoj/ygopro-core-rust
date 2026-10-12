//! Reindsurm Giriakarma
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 101403028;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[101403055], listed_series: &[SET_REINDSURM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // When your opponent activates a monster effect in the field or GY and you control this Ritual Summoned card (Quick Effect): You can negate the activation, then you can shuffle 1 monster each from both your opponent's field and GY into the Deck, also if either of them was a Dragon monster, double this card's ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_TODECK+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_TODECK + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card and a LIGHT "Reindsurm" monster are in your GY: You can add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.selfthcon)
    effect::set_condition(d, e2, Some(condition(selfthcon)));
    // e2:SetTarget(s.selfthtg)
    effect::set_target(d, e2, Some(cost_target(selfthtg)));
    // e2:SetOperation(s.selfthop)
    effect::set_operation(d, e2, Some(operation(selfthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,eg,ep,ev,re,r,rp)  (the condition of e1)
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect() and Chain.IsTriggeringLocation(ev,LOCATION_MZONE|LOCATION_GRAVE)
    //     and e:GetHandler():IsRitualSummoned() and Chain.IsNegatable(ev)
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[(LOCATION_MZONE | LOCATION_GRAVE) as i64]).truthy()
        && {
            let h = effect::get_handler(d, a.e).nil();
            card::is_ritual_summoned(d, h)
        }
        && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,2,1-tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 2, 1 - a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.NegateActivation(ev) then return end
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(aux.AND(Card.IsMonster,Card.IsAbleToDeck)),tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c))),
        a.tp,
        0,
        LOCATION_MZONE | LOCATION_GRAVE,
        NO_EX,
    );
    // if #g>=2 and g:GetClassCount(Card.GetLocation)==2 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if g.len() >= 2
        && group::get_class_count(d, &g, &|d, c| card::get_location(d, c) as i64) == 2
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetLocation),1,tp,HINTMSG_TODECK)
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(a.e),
            a.tp,
            Some(2),
            Some(2),
            Some(&aux::dpcheck(|d: &mut Duel, c: CardId| card::get_location(d, c) as i64)),
            1,
            a.tp,
            Some(HINTMSG_TODECK as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg~=2 then return end
        if sg.len() != 2 {
            return;
        }
        // local dragon_chk=sg:IsExists(Card.IsRace,1,nil,RACE_DRAGON)
        let dragon_chk = group::is_exists(d, &sg, &|d, c| card::is_race(d, c, RACE_DRAGON), 1, NO_EX);
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // if not dragon_chk then return end
        if !dragon_chk {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // Also if either of them was a Dragon monster, double this card's ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(c:GetAttack()*2)
            let atk = card::get_attack(d, c) as i64;
            effect::set_value(d, e1, atk * 2);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
// function s.selfthconfilter(c)
fn selfthconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsSetCard(SET_REINDSURM)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_set_card(d, c, &[SET_REINDSURM as u16])
}
// function s.selfthcon(e,tp,eg,ep,ev,re,r,rp)
fn selfthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.selfthconfilter,tp,LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&selfthconfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.selfthop(e,tp,eg,ep,ev,re,r,rp)
fn selfthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
