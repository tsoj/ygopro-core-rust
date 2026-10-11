//! Performapal Celestial Magician
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 58092907;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Special summon back your destroyed monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply effect based on the Types of monsters you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetLabel(id)
        effect::set_label(d, ge1, &[ID as i64]);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // ge1:SetOperation(aux.sumreg)
        effect::set_operation(d, ge1, Some(aux::sumreg()));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        // ge2:SetLabel(id)
        effect::set_label(d, ge2, &[ID as i64]);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
/// `s.spcfilter(c,e,tp)`
fn spcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ) and c:GetSummonLocation()==LOCATION_EXTRA and c:GetPreviousControler()==tp
    //   and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp))
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)
        && card::get_summon_location(d, c) == LOCATION_EXTRA
        && card::get_previous_controler(d, c) == tp
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != tp))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return #eg==1 and eg:IsExists(s.spcfilter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let (e, tp) = (a.e, a.tp);
    eg.len() == 1 && group::is_exists(d, &eg, &|d, x| spcfilter(d, x, e, tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.SpecialSummon(eg,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::special_summon(d, a.eg.clone().nil(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
    }
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0 and Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ+TYPE_PENDULUM),tp,LOCATION_MZONE,0,e:GetHandler())>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
        && duel::get_matching_group_count(d, Some(&faceup_special), a.tp, LOCATION_MZONE, 0, h.into()) > 0
}
/// `aux.FaceupFilter(Card.IsType,TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ+TYPE_PENDULUM)`
fn faceup_special(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ + TYPE_PENDULUM)
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(...),tp,LOCATION_MZONE,0,c)
    let g = duel::get_matching_group(d, Some(&faceup_special), a.tp, LOCATION_MZONE, 0, c.into());
    // if not c:IsRelateToEffect(e) or #g==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || g.is_empty() {
        return;
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_FUSION) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_FUSION), 1, NO_EX) {
        // This card can attack directly
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_SYNCHRO), 1, NO_EX) {
        // The opponent cannot activate monster effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetTargetRange(0,1)
        effect::set_target_range(d, e2, 0, 1);
        // e2:SetValue(s.aclimit)
        effect::set_value_fn(d, e2, lua_fn(aclimit));
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_XYZ) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_XYZ), 1, NO_EX) {
        // This card's ATK becomes double its original ATK
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
        // e3:SetValue(c:GetBaseAttack()*2)
        let base = card::get_base_attack(d, c) as i64;
        effect::set_value(d, e3, base * 2);
        // e3:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
    // if g:IsExists(Card.IsType,1,nil,TYPE_PENDULUM) then
    if group::is_exists(d, &g, &|d, x| card::is_type(d, x, TYPE_PENDULUM), 1, NO_EX) {
        // Search 1 Pendulum Monster during the End Phase
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e4:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
        // e4:SetCountLimit(1)
        effect::set_count_limit(d, e4, 1, 0, 0, 0);
        // e4:SetOperation(s.thop)
        effect::set_operation(d, e4, Some(operation(thop)));
        // e4:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, a.tp);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    let re = a.e(1);
    effect::is_monster_effect(d, re)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
