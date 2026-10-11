//! Gouki The Tyrant Ogre
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 7782069;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Fusion Materials: 1 "Gouki" Link Monster + 1 Warrior, Dinosaur, or Cyberse monster
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR|RACE_DINOSAUR|RACE_CYBERSE))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| matfilter(d, c))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_race_ctx(d, c, RACE_WARRIOR | RACE_DINOSAUR | RACE_CYBERSE, Some(fc), st, Some(tp))
            })),
        ],
    );
    // --Register the Link Rating of the materials used for its Fusion Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(s.valcheck)
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // --Destroy cards on the field up to the total Link Rating of the "Gouki" Link Monsters used as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() and e:GetLabel()>0 end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, c) && effect::get_label(d, a.e) > 0
    })));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // e0:SetLabelObject(e1)
    effect::set_label_object(d, e0, Value::Effect(e1));
    // --If this card battles, your opponent cannot activate cards or effects until the end of the Damage Step
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCondition(function(e) return Duel.GetAttacker()==e:GetHandler() or Duel.GetAttackTarget()==e:GetHandler() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::get_attacker(d) == Some(h) || duel::get_attack_target(d) == Some(h)
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GOUKI) and c:IsLinkMonster()
    card::is_set_card(d, c, &[SET_GOUKI as u16]) && card::is_link_monster(d, c)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local total=g:Filter(Card.IsSetCard,nil,SET_GOUKI):GetSum(Card.GetLink)
    let gouki = group::filter(d, &g, &|d, x| card::is_set_card(d, x, &[SET_GOUKI as u16]), NO_EX);
    let total = group::get_sum(d, &gouki, &|d, x| card::get_link(d, x) as i64);
    // local link3=g:IsExists(Card.IsLinkAbove,1,nil,3) and 100 or 0
    let link3 = if group::is_exists(d, &g, &|d, x| card::is_link_above(d, x, 3), 1, NO_EX) { 100 } else { 0 };
    // e:GetLabelObject():SetLabel(total,link3)
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[total, link3]);
    // (no return value: nil, read as 0)
    0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local total,link3=e:GetLabel()
    let labels = effect::get_labels(d, e);
    let total = labels.first().copied().unwrap_or(0);
    let link3 = labels.get(1).copied().unwrap_or(0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,total,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, total as u16, false, NO_EX).nil();
    // if link3==100 then
    if link3 == 100 {
        // Duel.SetChainLimit(function(te) return not Duel.GetTargetCards(e):IsContains(te:GetHandler()) end)
        duel::set_chain_limit(d, lua_fn(move |d, a: Args| -> bool {
            let te = a.e(0);
            let h = effect::get_handler(d, te).nil();
            let tg = duel::get_target_cards(d, e).nil();
            !group::is_contains(&tg, h)
        }));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
