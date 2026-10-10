//! Destiny HERO - Dreadmaster
use super::prelude::*;

pub const ID: u32 = 40591390;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75041269], listed_series: &[SET_DESTINY_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Special Summoned by "Clock Tower Prison": Destroy all monsters you control, except "Destiny HERO" monsters, also, after that, you can Special Summon up to 2 "Destiny HERO" monsters from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:GetHandler():IsCode(75041269,4663194) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        card::is_code(d, h, &[75041269, 4663194])
    })));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // After this card is Special Summoned, for the rest of this turn, "Destiny HERO" monsters you control cannot be destroyed, also you take no battle damage when they battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // The ATK/DEF of this card are equal to the combined original ATK of all other "Destiny HERO" monsters you control
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE);
    // e3a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3a:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e3a, EFFECT_SET_ATTACK);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetValue(function(e,c) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DESTINY_HERO),c:GetControler(),LOCATION_MZONE,0,c):GetSum(Card.GetBaseAttack) end)
    effect::set_value_fn(d, e3a, effect_card_value(atkval));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e3b, EFFECT_SET_DEFENSE);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `function(e,c) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DESTINY_HERO),c:GetControler(),LOCATION_MZONE,0,c):GetSum(Card.GetBaseAttack) end`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_DESTINY_HERO as u16])),
        p,
        LOCATION_MZONE,
        0,
        c.into(),
    );
    group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsSetCard(SET_DESTINY_HERO)
    card::is_facedown(d, c) || !card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DESTINY_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local break_chk=false
    let mut break_chk = false;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // break_chk=true
        break_chk = true;
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // local mmz_count=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let (mut mmz_count, _) = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
    // if mmz_count>0 and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if mmz_count > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
            a.tp,
            LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // mmz_count=math.min(mmz_count,2)
        mmz_count = mmz_count.min(2);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then mmz_count=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            mmz_count = 1;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,mmz_count,nil,e,tp)
        let sg = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
            a.tp,
            LOCATION_GRAVE,
            0,
            1,
            mmz_count as u16,
            false,
            NO_EX,
        )
        .nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // For the rest of this turn, "Destiny HERO" monsters you control cannot be destroyed, also you take no battle damage when they battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_DESTINY_HERO) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
