//! Vaylantz Wave - Master Phase
use super::prelude::*;

pub const ID: u32 = 84281045;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAYLANTZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAYLANTZ) and c:IsLevelAbove(5) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::is_level_above(d, c, 5) && card::is_faceup(d, c)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MMZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsType(TYPE_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MMZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_faceup(d, chkc)
            && card::is_type(d, chkc, TYPE_EFFECT);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local not_dmg_step=Duel.GetCurrentPhase()~=PHASE_DAMAGE
    let not_dmg_step = duel::get_current_phase(d) != PHASE_DAMAGE;
    // local b1=Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) and (not_dmg_step or not Duel.IsDamageCalculated())
    let b1 = duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && (not_dmg_step || !duel::is_damage_calculated(d));
    // local b2=c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) and not_dmg_step
    //   and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MMZONE,1,nil)
    let b2 = card::is_able_to_grave_as_cost(d, c)
        && card::is_status(d, c, STATUS_EFFECT_ENABLED as u32)
        && not_dmg_step
        && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, TYPE_EFFECT)), tp, 0, LOCATION_MMZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // local prop=EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP
    let prop = EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP;
    if op == 1 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // e:SetProperty(prop)
        effect::set_property(d, a.e, prop, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(prop+EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, prop + EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.SendtoGrave(c,REASON_COST)
        duel::send_to_grave(d, c, REASON_COST, None, None);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // local tc=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MMZONE,1,1,nil,tp):GetFirst()
        let sel = duel::select_target(
            d,
            tp,
            Some(&|d, x| card::is_faceup(d, x) && card::is_type(d, x, TYPE_EFFECT)),
            tp,
            0,
            LOCATION_MMZONE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let tc = group::get_first(d, &sel).nil();
        // local dc=Duel.GetFieldCard(1-tp,LOCATION_SZONE,tc:GetSequence())
        let seq = card::get_sequence(d, tc);
        let dc = duel::get_field_card(d, 1 - tp, LOCATION_SZONE, seq);
        // if dc then
        if let Some(dc) = dc {
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dc,1,PLAYER_NONE,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(dc)), 1, PLAYER_NONE as u8, 0);
        }
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // local g=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(1200)
            effect::set_value(d, e1, 1200);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    } else if op == 2 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if not (tc:IsRelateToEffect(e) and tc:IsControler(1-tp) and not tc:IsImmuneToEffect(e)) then return end
        if !(card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp) && !card::is_immune_to_effect(d, tc, a.e)) {
            return;
        }
        // local seq=tc:GetSequence()
        let seq = card::get_sequence(d, tc);
        // local dc=Duel.GetFieldCard(1-tp,LOCATION_SZONE,seq)
        let dc = duel::get_field_card(d, 1 - a.tp, LOCATION_SZONE, seq);
        // if dc then Duel.Destroy(dc,REASON_RULE) end
        if let Some(dc) = dc {
            duel::destroy(d, dc, REASON_RULE, None, None);
        }
        // if Duel.CheckLocation(1-tp,LOCATION_SZONE,seq)
        //   and Duel.MoveToField(tc,tp,1-tp,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard(),1<<seq) then
        let is_monster = card::is_monster_card(d, tc);
        if duel::check_location(d, 1 - a.tp, LOCATION_SZONE, seq)
            && duel::move_to_field(d, tc, a.tp, 1 - a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, Some(1 << seq))
        {
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
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
