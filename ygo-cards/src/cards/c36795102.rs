//! Crystal Beast Rainbow Dragon
use super::prelude::*;

pub const ID: u32 = 36795102;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_CRYSTAL_BEAST,SET_ULTIMATE_CRYSTAL}
    listed_series: &[SET_CRYSTAL_BEAST, SET_ULTIMATE_CRYSTAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place itself in the Spell & Trap Zone as a Continuous Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e1:SetCondition(s.repcon)
    effect::set_condition(d, e1, Some(lua_fn(repcon)));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(lua_fn(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.selfspcon)
    effect::set_condition(d, e2, Some(condition(selfspcon)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Level 4 or lower "Crystal Beast" monster from your Deck and search 1 "Ultimate Crystal" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsContinuousSpell() end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::is_continuous_spell(d, h)
    })));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.spthtg)
    effect::set_target(d, e3, Some(cost_target(spthtg)));
    // e3:SetOperation(s.spthop)
    effect::set_operation(d, e3, Some(operation(spthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.repcon(e)`
fn repcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_DESTROY)
}
/// `s.repop(e,tp,...)`
fn repop(d: &mut Duel, a: Args) {
    let (e, tp) = (a.e(0), a.p(1));
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Treated as a Continuous Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, c, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(e), 0, tp, 0, 0);
}
/// `s.selfspcon(e,tp,...)`
fn selfspcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local dc=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // return Duel.GetAttacker():IsSetCard(SET_CRYSTAL_BEAST) or (dc and dc:IsFaceup() and dc:IsSetCard(SET_CRYSTAL_BEAST))
    let attacker = duel::get_attacker(d).nil();
    if card::is_set_card(d, attacker, &[SET_CRYSTAL_BEAST as u16]) {
        return true;
    }
    match dc {
        Some(dc) => card::is_faceup(d, dc) && card::is_set_card(d, dc, &[SET_CRYSTAL_BEAST as u16]),
        None => false,
    }
}
/// `s.selfsptg(e,tp,...,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
    true
}
/// `s.selfspop(e,tp,...)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c)
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_ULTIMATE_CRYSTAL) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_ULTIMATE_CRYSTAL as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.spthtg(e,tp,...,chk)`
fn spthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spthop(e,tp,...)`
fn spthop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Negate its effects
            // sc:NegateEffects(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            card::negate_effects(d, sc, h, None, false, None);
        }
    }
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let hg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #hg>0 then
    if !hg.is_empty() {
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg);
    }
}
