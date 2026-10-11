//! Summon Reactor ・SK
use super::prelude::*;

pub const ID: u32 = 89493368;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15175429, 16898077, 52286175], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // negate attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetCondition(s.nacon)
    effect::set_condition(d, e4, Some(condition(nacon)));
    // e4:SetTarget(s.natg)
    effect::set_target(d, e4, Some(cost_target(natg)));
    // e4:SetOperation(s.naop)
    effect::set_operation(d, e4, Some(operation(naop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCost(s.spcost)
    effect::set_cost(d, e5, Some(cost_target(spcost)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsControler,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, cc| card::is_controler(d, cc, 1 - a.tp), 1, NO_EX)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 800);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.Damage(1-tp,800,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 800, REASON_EFFECT, false, None);
    }
}
/// `s.nacon(e,tp,eg,ep,ev,re,r,rp)`
fn nacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(1-tp) and e:GetHandler():GetFlagEffect(id)~=0
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_controler(d, tc, 1 - a.tp) && card::get_flag_effect(d, h, ID) != 0
}
/// `s.natg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e));
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.nil());
    true
}
/// `s.naop(e,tp,eg,ep,ev,re,r,rp)`
fn naop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.NegateAttack()
        duel::negate_attack(d);
    }
}
/// `s.rescon(ct)`: the rescon of `aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(dec),...)`
fn rescon(ct: i32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, e, tp, mg, c| {
        // return aux.ChkfMMZ(1-ct)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg)
        let chkf = aux::chkf_mmz(1 - ct);
        let r = chkf(d, sg, e, tp, mg, c).0 && group::is_exists(d, sg, &|d, cc| chk(d, cc, sg), 1, NO_EX);
        (r, false)
    }
}
/// `s.chk(c,sg)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsCode(15175429) and sg:IsExists(Card.IsCode,1,c,52286175)
    card::is_code(d, c, &[15175429]) && group::is_exists(d, sg, &|d, cc| card::is_code(d, cc, &[52286175]), 1, c.into())
}
/// `s.spcfilter(c,code)`
fn spcfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[code]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g1=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_ONFIELD,0,c,15175429)
    let g1 = duel::get_matching_group(d, Some(&|d, cc| spcfilter(d, cc, 15175429)), a.tp, LOCATION_ONFIELD, 0, c.into());
    // local g2=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_ONFIELD,0,c,52286175)
    let g2 = duel::get_matching_group(d, Some(&|d, cc| spcfilter(d, cc, 52286175)), a.tp, LOCATION_ONFIELD, 0, c.into());
    // local g=g1:Clone()
    let mut g = g1.clone();
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // local dec=c:GetSequence()<5 and 1 or 0
    let dec: i32 = if card::get_sequence(d, c) < 5 { 1 } else { 0 };
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and e:GetHandler():IsAbleToGraveAsCost()
    //     and #g1>0 and #g2>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(dec),0) end
    if a.chk == Some(0) {
        let rc = rescon(dec);
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -3
            && card::is_able_to_grave_as_cost(d, c)
            && !g1.is_empty()
            && !g2.is_empty()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rc), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon(dec),1,tp,HINTMSG_TOGRAVE)
    let rc = rescon(dec);
    let mut sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rc),
        1,
        a.tp,
        Some(HINTMSG_TOGRAVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // sg:AddCard(c)
    group::add_card(&mut sg, &CardSet::from_card(c));
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(16898077) and c:IsCanBeSpecialSummoned(e,0,tp,true,true)
    card::is_code(d, c, &[16898077]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, cc| spfilter(d, cc, e, tp)),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(move |d, cc| spfilter(d, cc, e, tp))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon(d, g.clone(), 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
        // g:GetFirst():CompleteProcedure()
        let tc = group::get_first(d, &g).nil();
        card::complete_procedure(d, tc);
    }
}
