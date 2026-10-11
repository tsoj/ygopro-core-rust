//! Flame Swordsrealm
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 73714736;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FLAME_SWORDSMAN], ..ScriptInfo::DEFAULT };

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
    // Opponent cannot activate cards or effects when you Normal Summon a FIRE Warrior monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.limop)
    effect::set_operation(d, e2, Some(operation(limop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Flame Swordsman" from your Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Decrease the ATK of a Warrior monster by 1000 ATK and increase the ATK of other monsters by 1000
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e5, CATEGORY_ATKCHANGE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e5, EVENT_ATTACK_ANNOUNCE);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.atktg)
    effect::set_target(d, e5, Some(cost_target(atktg)));
    // e5:SetOperation(s.atkop)
    effect::set_operation(d, e5, Some(operation(atkop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.nsfilter(c,sp)`
fn nsfilter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_WARRIOR) and c:IsFaceup() and c:IsSummonPlayer(sp)
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_WARRIOR) && card::is_faceup(d, c) && card::is_summon_player(d, c, sp)
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if eg:IsExists(s.nsfilter,1,nil,tp) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| nsfilter(d, c, tp), 1, NO_EX) {
        // Duel.SetChainLimitTillChainEnd(function(e,_rp,_tp) return _tp==_rp end)
        duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, a: Args| a.p(2) == a.p(1)));
    }
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsAbleToGraveAsCost() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_monster(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,mc)`: `mc` is the material card (nil when the filter gets no fourth argument)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsCode(CARD_FLAME_SWORDSMAN) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false) and c:CheckFusionMaterial()
    if !card::is_code(d, c, &[CARD_FLAME_SWORDSMAN]) {
        return false;
    }
    let ex = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
        && card::check_fusion_material(d, c, None, None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
/// `s.atkfilter(c,tp)`
fn atkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WARRIOR) and c:IsAttackAbove(1000)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,c)
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_attack_above(d, c, 1000)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atkfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| atkfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,#g,tp,1000)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g), n, tp, 1000);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,tc)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, tc.into());
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // It loses exactly 1000 ATK
    // if tc:IsAttackAbove(1000) and tc:UpdateAttack(-1000,RESET_PHASE|PHASE_END,c)==-1000 then
    if card::is_attack_above(d, tc, 1000) && card::update_attack(d, tc, -1000, Some(RESET_PHASE | PHASE_END), Some(c), None) == -1000 {
        // All other monsters gain 1000 ATK
        // for sc in g:Iter() do
        for sc in group::iter(d, &g) {
            // sc:UpdateAttack(1000,RESET_PHASE|PHASE_END,c)
            card::update_attack(d, sc, 1000, Some(RESET_PHASE | PHASE_END), Some(c), None);
        }
    }
}
