//! Particle Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 39261576;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEM_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_GEM_KNIGHT),Fusion.OnFieldMat,nil,nil,nil,s.stage2)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]))),
            matfilter: Some(fusion::on_field_mat()),
            stage2: Some(Arc::new(stage2)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk != 1 {
        return;
    }
    // e:SetLabelObject(tc)
    let label = match tc {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, e, label);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e1, EVENT_CHAIN_END);
    // e1:SetOperation((function(p,_e) return function(e) Duel.RaiseSingleEvent(_e:GetHandler(),EVENT_CUSTOM+id,_e,0,p,p,0) e:Reset() end end)(tp,e))
    let p = tp;
    let fe = e;
    effect::set_operation(
        d,
        e1,
        Some(lua_fn(move |d, a: Args| {
            let fh = effect::get_handler(d, fe).nil();
            duel::raise_single_event(d, fh, EVENT_CUSTOM + ID, Some(fe), 0, p, p, 0);
            effect::reset(d, a.e(0));
            Value::Nil
        })),
    );
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_location(d, h, LOCATION_GRAVE) && card::is_able_to_remove(d, h, None, None, None);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=e:GetLabelObject():GetLabelObject()
    let fe = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, fe).as_card().nil();
    // local mat=tc:GetMaterial()
    let mat = card::get_material(d, tc);
    // if chkc then return chkc:IsSetCard(SET_GEM_KNIGHT) and mat:IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_set_card(d, chkc, &[SET_GEM_KNIGHT as u16]) && group::is_contains(&mat, chkc);
    }
    // if chk==0 then return mat:IsExists(Card.IsSetCard,1,nil,SET_GEM_KNIGHT) end
    if a.chk == Some(0) {
        return group::is_exists(d, &mat, &|d, c| card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=mat:FilterSelect(tp,Card.IsSetCard,1,1,nil,SET_GEM_KNIGHT)
    let g = group::filter_select(d, &mat, tp, &|d, c| card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]), 1, 1, false, NO_EX).nil();
    // tc:CreateEffectRelation(e)
    card::create_effect_relation(d, tc, a.e);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local sc=e:GetLabelObject():GetLabelObject()
    let fe = effect::get_label_object(d, a.e).as_effect().nil();
    let sc = effect::get_label_object(d, fe).as_card().nil();
    // if not sc:IsRelateToEffect(e) or sc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, sc, a.e) || card::is_facedown(d, sc) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(tc:GetAttack())
    let atk = card::get_attack(d, tc) as i64;
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // sc:RegisterEffect(e1)
    card::register_effect(d, sc, e1, false);
}
