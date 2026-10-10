//! Oneiros, the Dream Mirror Tormentor
use super::prelude::*;

pub const ID: u32 = 37678339;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DREAM_MIRROR_TERROR, CARD_DREAM_MIRROR_JOY, 35187185],
    listed_series: &[SET_DREAM_MIRROR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // While face-up on the field, this card is also LIGHT-Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(ATTRIBUTE_LIGHT)
    effect::set_value(d, e1, ATTRIBUTE_LIGHT as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate an activated monster effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re) return re:IsMonsterEffect() and s.fzonecon(CARD_DREAM_MIRROR_TERROR) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(function(e,tp,eg,ep,ev) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Oneiros, the Dream Mirror Erlking" from your Extra Deck in Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(function() return s.fzonecon(CARD_DREAM_MIRROR_JOY) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,sp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return c:IsSetCard(SET_DREAM_MIRROR,fc,sumtype,sp) and (not sg or sg:FilterCount(aux.TRUE,c)==0
    //   or not sg:IsExists(Card.IsAttribute,1,c,c:GetAttribute(),fc,sumtype,sp))
    if !card::is_set_card_ctx(d, c, &[SET_DREAM_MIRROR as u16], Some(a.fc), a.sumtype, Some(a.tp)) {
        return false;
    }
    match a.sg {
        None => true,
        Some(sg) => {
            if group::filter_count(d, sg, &|_, _| true, c.into()) == 0 {
                return true;
            }
            let attr = card::get_attribute_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp));
            !group::is_exists(d, sg, &|d, x| card::is_attribute_ctx(d, x, attr, Some(a.fc), a.sumtype, Some(a.tp)), 1, c.into())
        }
    }
}
/// `s.fzonecon(code)`
fn fzonecon(d: &mut Duel, code: u32) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,code),0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[code])), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
// function(e,tp,eg,ep,ev,re) return re:IsMonsterEffect() and s.fzonecon(CARD_DREAM_MIRROR_TERROR) end
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && fzonecon(d, CARD_DREAM_MIRROR_TERROR)
}
// function(e,tp,eg,ep,ev,re,r,rp) return s.fzonecon(CARD_DREAM_MIRROR_JOY) end
fn cond3(d: &mut Duel, _a: &EffArgs) -> bool {
    fzonecon(d, CARD_DREAM_MIRROR_JOY)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
// function(e,tp,eg,ep,ev) Duel.NegateEffect(ev) end
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
/// `s.spfilter(c,e,tp,rc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: CardId) -> bool {
    // return c:IsCode(35187185) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //   and Duel.GetLocationCountFromEx(tp,tp,rc,c)>0
    card::is_code(d, c, &[35187185])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), rc.into(), duel::ExtraCard::Card(c), None).0 > 0
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, h)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let h = effect::get_handler(d, e).nil();
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, h)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
