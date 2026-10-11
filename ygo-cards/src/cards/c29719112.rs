//! Cyber Pharos
use super::prelude::*;

pub const ID: u32 = 29719112;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[37630732], ..ScriptInfo::DEFAULT };

/// `params = {aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE)}` of the Fusion Summon effect
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff { fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_MACHINE))), ..Default::default() }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(lua_fn(spcon1)));
    // e1:SetTarget(s.sptg1)
    effect::set_target(d, e1, Some(lua_fn(sptg1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(lua_fn(spop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion summon
    // local params = {aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE)}
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(Fusion.SummonEffTG(table.unpack(params)))
    let tg = fusion::summon_eff_tg(d, fusion_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(table.unpack(params)))
    let op = fusion::summon_eff_op(d, fusion_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.spcon1(e,c)
fn spcon1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsRace,1,false,1,true,c,c:GetControler(),nil,false,e:GetHandler(),RACE_MACHINE)
    let h = effect::get_handler(d, e).nil();
    duel::check_release_group(
        d,
        tp,
        Some(&|d, c| card::is_race(d, c, RACE_MACHINE)),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, ..Default::default() },
        h.into(),
        false,
    )
}
// function s.sptg1(e,tp,eg,ep,ev,re,r,rp,c)
fn sptg1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(8);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsRace,1,1,false,true,true,c,nil,nil,false,e:GetHandler(),RACE_MACHINE)
    let h = effect::get_handler(d, e).nil();
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, c| card::is_race(d, c, RACE_MACHINE)),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: c, toplayer: None, zone: None, use_oppo: false, ..Default::default() },
        h.into(),
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
// function s.spop1(e,tp,eg,ep,ev,re,r,rp,c)
fn spop1(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // g:DeleteGroup()
    }
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsType(TYPE_FUSION) and c:IsReason(REASON_BATTLE)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, tp) && card::is_type(d, c, TYPE_FUSION) && card::is_reason(d, c, REASON_BATTLE)
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(37630732) and c:IsAbleToHand()
    card::is_code(d, c, &[37630732]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
