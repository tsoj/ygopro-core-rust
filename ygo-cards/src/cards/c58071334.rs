//! Snake-Eyes Doomed Dragon
use super::prelude::*;

pub const ID: u32 = 58071334;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // Fusion Materials: 1 "Snake-Eye" monster + 1 Illusion monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SNAKE_EYE),aux.FilterBoolFunctionEx(Card.IsRace,RACE_ILLUSION))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_SNAKE_EYE as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_ILLUSION, Some(fc), st, Some(tp)))),
        ],
    );
    // Alternative Summon Procedure
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCondition(s.hspcon)
    effect::set_condition(d, e0, Some(lua_fn(hspcon)));
    // e0:SetTarget(s.hsptg)
    effect::set_target(d, e0, Some(lua_fn(hsptg)));
    // e0:SetOperation(s.hspop)
    effect::set_operation(d, e0, Some(lua_fn(hspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Place 1 monster from the field in the Spell & Trap Zone as a Continuous Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.hspfilter(c)
fn hspfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonsterCard() and c:IsFaceup() and c:IsAbleToGraveAsCost() and c:IsCanBeFusionMaterial()
    card::is_monster_card(d, c) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c) && card::is_can_be_fusion_material(d, c, None, None, None)
}
// function s.hspcon(e,c)  (EFFECT_SPSUMMON_PROC condition)
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and Duel.IsExistingMatchingCard(s.hspfilter,tp,LOCATION_STZONE,0,2,nil)
    duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && duel::is_existing_matching_card(d, Some(&hspfilter), tp, LOCATION_STZONE, 0, 2, NO_EX)
}
// function s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)  (EFFECT_SPSUMMON_PROC target)
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hspfilter,tp,LOCATION_STZONE,0,2,2,true,nil)
    let Some(g) = duel::select_matching_card(d, tp, Some(&hspfilter), tp, LOCATION_STZONE, 0, 2, 2, true, NO_EX) else {
        // if not g then return false end
        return false;
    };
    // g:KeepAlive()
    // e:SetLabelObject(g)
    effect::set_label_object(d, e, Value::Group(g));
    // return true
    true
}
// function s.hspop(e,tp,eg,ep,ev,re,r,rp,c)  (EFFECT_SPSUMMON_PROC operation)
fn hspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e(0)).as_group().cloned() else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g, REASON_COST | REASON_MATERIAL, None, None);
    // g:DeleteGroup()
}
// function s.plfilter(c)
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // local p=c:GetOwner()
    let p = card::get_owner(d, c);
    // return c:IsFaceup() and Duel.GetLocationCount(p,LOCATION_SZONE)>0 and not c:IsForbidden() and c:CheckUniqueOnField(p)
    card::is_faceup(d, c)
        && duel::get_location_count(d, p, LOCATION_SZONE as u8, None, None, None).0 > 0
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, p, None, None)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.plfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && plfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.plfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&plfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // Duel.SelectTarget(tp,s.plfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&plfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // local p=tc:GetOwner()
    let p = card::get_owner(d, tc);
    // if Duel.GetLocationCount(p,LOCATION_SZONE)==0 then
    if duel::get_location_count(d, p, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        // Duel.SendtoGrave(tc,REASON_RULE,nil,PLAYER_NONE)
        duel::send_to_grave(d, tc, REASON_RULE, None, Some(PLAYER_NONE as u8));
    // elseif tc:CheckUniqueOnField(p) and Duel.MoveToField(tc,tp,p,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
    } else if card::check_unique_on_field(d, tc, p, None, None)
        && duel::move_to_field(d, tc, tp, p, LOCATION_SZONE as u16, POS_FACEUP as u8, card::is_monster_card(d, tc), None)
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
        // e1:SetReset((RESET_EVENT|RESETS_STANDARD)&~RESET_TURN_SET)
        effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) & !RESET_TURN_SET, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
