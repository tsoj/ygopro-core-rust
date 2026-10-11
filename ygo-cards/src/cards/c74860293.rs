//! Junk Destroyer
use super::prelude::*;

pub const ID: u32 = 74860293;

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_JUNK_SYNCHRON],
    listed_names: &[CARD_JUNK_SYNCHRON],
    material_setcode: &[SET_SYNCHRON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Junk Synchron" + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,s.tunerfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| tunerfilter(d, c, sc, st, tp))), 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Destroy cards on the field,up to the number of non-Tuner monsters used for the Synchro Summon of this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(synchro_cond)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn synchro_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_synchro_summoned(d, h)
}
// s.tunerfilter(c,lc,stype,tp)
fn tunerfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // c:IsSummonCode(lc,stype,tp,CARD_JUNK_SYNCHRON) or c:IsHasEffect(20932152)
    card::is_summon_code(d, c, lc, stype, tp, &[CARD_JUNK_SYNCHRON]) || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // local ct=e:GetHandler():GetMaterialCount()-1
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_material_count(d, h) as i64 - 1;
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if group::get_count(&tg) > 0 {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
