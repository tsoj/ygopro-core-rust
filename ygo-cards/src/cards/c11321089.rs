//! Guardian Chimera
use super::prelude::*;

pub const ID: u32 = 11321089;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_POLYMERIZATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local eff=Fusion.AddProcMix(c,true,false,s.ffilter1,s.ffilter2,s.ffilter3)
    let eff = fusion::add_proc_mix(
        d,
        c,
        true,
        false,
        vec![
            fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter1(d, c, a))),
            fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter2(d, c, a))),
            fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter3(d, c, a))),
        ],
    );
    // if not c:IsStatus(STATUS_COPYING_EFFECT) then eff[1]:SetValue(s.matfilter) end
    if !card::is_status(d, c, STATUS_COPYING_EFFECT) {
        let e = eff.nil();
        effect::set_value_fn(d, e, lua_fn(matfilter));
    }
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // Keep track of how many materials are used from the hand and field
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e0, lua_fn(valcheck));
    card::register_effect(d, c, e0, false);
    // Draw cards equal to the number of cards used as material from the hand, and if you do, destroy cards your opponent controls equal to the number of cards used as material from the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_label_object(d, e1, Value::Effect(e0));
    effect::set_condition(d, e1, Some(condition(drdes_cond)));
    effect::set_target(d, e1, Some(cost_target(drdestg)));
    effect::set_operation(d, e1, Some(operation(drdesop)));
    card::register_effect(d, c, e1, false);
    // While "Polymerization" is in your GY, your opponent cannot target this card with card effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    effect::set_value_fn(d, e2, aux::tgoval());
    card::register_effect(d, c, e2, false);
}
/// `s.summon_code(c,code,fc,tp)`: `c:GetCode(fc,sumtype,tp)` as its first value (nil if none).
fn first_summon_code(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> Option<u32> {
    let mut codes = std::collections::BTreeSet::new();
    c.get_summon_code(d, &mut codes, Some(fc), sumtype, tp);
    codes.iter().next().copied()
}
/// `s.fusfilter(c,code,fc,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, tp: u8) -> bool {
    // return c:IsSummonCode(fc,SUMMON_TYPE_FUSION,tp,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), SUMMON_TYPE_FUSION as u64, tp, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `not sg or not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,tp),fc,sumtype,tp)`
fn no_fusion_partner(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    match a.sg {
        None => true,
        Some(sg) => {
            // a nil code is passed as 0 to IsSummonCode (the core reads the argument as an integer)
            let code = first_summon_code(d, c, a.fc, a.sumtype, a.tp).unwrap_or(0);
            let fc = a.fc;
            let sumtype = a.sumtype;
            !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, sumtype as u8), 1, c.into())
        }
    }
}
/// `s.ffilter1(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter1(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    (a.sumtype == MATERIAL_FUSION as u64 || card::is_location(d, c, LOCATION_HAND)) && no_fusion_partner(d, c, a)
}
/// `s.ffilter2(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter2(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    (a.sumtype == MATERIAL_FUSION as u64 || card::is_location(d, c, LOCATION_ONFIELD)) && no_fusion_partner(d, c, a)
}
/// `s.ffilter3(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter3(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    no_fusion_partner(d, c, a)
}
/// `s.matfilter(c,fc,sub,sub2,mg,sg,tp,contact,sumtype)`
fn matfilter(d: &mut Duel, a: Args) -> bool {
    let (c, fc) = (a.c(0), a.c(1));
    let tp = a.p(6);
    let sumtype = a.int(8) as u64;
    if (sumtype & SUMMON_TYPE_FUSION as u64) > 0 && card::is_location(d, fc, LOCATION_EXTRA) {
        return card::is_location(d, c, LOCATION_HAND | LOCATION_ONFIELD) && card::is_controler(d, c, tp);
    }
    true
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // local hc=mg:FilterCount(Card.IsLocation,nil,LOCATION_HAND)
    let hc = group::filter_count(d, &mg, &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    // local fc=mg:FilterCount(Card.IsLocation,nil,LOCATION_ONFIELD)
    let fc = group::filter_count(d, &mg, &|d, x| card::is_location(d, x, LOCATION_ONFIELD), NO_EX);
    // e:SetLabel(hc,fc)
    effect::set_label(d, e, &[hc as i64, fc as i64]);
    Value::Nil
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of e1: `e:GetHandler():IsFusionSummoned() and re and re:IsSpellEffect()`
fn drdes_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_fusion_summoned(d, h) {
        return false;
    }
    match a.re {
        Some(re) => effect::is_spell_effect(d, re),
        None => false,
    }
}
/// `local hc,fc=e:GetLabelObject():GetLabel()`
fn labels_of(d: &mut Duel, e: EffectId) -> (i64, i64) {
    let lo = effect::get_label_object(d, e).as_effect().nil();
    let v = effect::get_labels(d, lo);
    (*v.first().nil(), *v.get(1).nil())
}
/// `s.drdestg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drdestg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let (hc, fc) = labels_of(d, a.e);
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return hc > 0 && fc > 0 && duel::is_player_can_draw(d, tp, hc as u32) && dg.len() as i64 >= fc;
    }
    duel::set_target_player(d, tp);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, hc as i32);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), fc as u8, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.drdesop(e,tp,eg,ep,ev,re,r,rp)`
fn drdesop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let (hc, fc) = labels_of(d, a.e);
    if duel::draw(d, p, hc as u32, REASON_EFFECT) as i64 == hc {
        let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
        if (g.len() as i64) < fc {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let dg = group::select(d, &g, tp, fc as u16, fc as u16, false, NO_EX).nil();
        if !dg.is_empty() {
            duel::hint_selection(d, dg.clone(), true);
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
/// `function(e) return Duel.IsExistingMatchingCard(Card.IsCode,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil,CARD_POLYMERIZATION) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_POLYMERIZATION])), p, LOCATION_GRAVE, 0, 1, NO_EX)
}
